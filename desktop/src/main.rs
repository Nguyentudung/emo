slint::include_modules!();

use slint::{ComponentHandle, Image, Rgba8Pixel, SharedPixelBuffer, Weak};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(0);

struct AppState {
    files: Vec<PathBuf>,
    current_index: isize,
    rotation: i32,
    original_width: u32,
    original_height: u32,
}

impl AppState {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            current_index: -1,
            rotation: 0,
            original_width: 0,
            original_height: 0,
        }
    }

    fn current_path(&self) -> Option<&PathBuf> {
        if self.current_index >= 0 && (self.current_index as usize) < self.files.len() {
            Some(&self.files[self.current_index as usize])
        } else {
            None
        }
    }
}

fn load_and_set_image(weak_window: Weak<MainWindow>, path: PathBuf, state: Arc<Mutex<AppState>>) {
    let req_id = REQUEST_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;

    std::thread::spawn(move || {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        match image_engine::decode_to_rgba(&path) {
            Ok(decoded) => {
                let width = decoded.width;
                let height = decoded.height;
                let rgba = decoded.rgba;

                // Check if a newer request arrived during decoding
                if req_id != REQUEST_COUNTER.load(Ordering::SeqCst) {
                    return;
                }

                // Construct pixel buffer on the background worker thread, eliminating UI thread latency
                let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(width, height);
                let slice = buffer.make_mut_slice();
                for (i, pixel) in rgba.chunks_exact(4).enumerate() {
                    if i < slice.len() {
                        slice[i] = Rgba8Pixel {
                            r: pixel[0],
                            g: pixel[1],
                            b: pixel[2],
                            a: pixel[3],
                        };
                    }
                }

                // Store original dimensions
                if let Ok(mut s) = state.lock() {
                    s.original_width = width;
                    s.original_height = height;
                }

                let _ = weak_window.upgrade_in_event_loop(move |win| {
                    if req_id != REQUEST_COUNTER.load(Ordering::SeqCst) {
                        return;
                    }
                    let image = Image::from_rgba8(buffer);
                    win.set_current_image(image);
                    win.set_has_image(true);
                    win.set_error_text("".into());
                    win.set_current_filename(file_name.into());
                    win.set_image_dimensions(format!("{} × {}", width, height).into());
                    win.set_metadata_folder(
                        path.parent()
                            .and_then(|p| p.file_name())
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default()
                            .into(),
                    );
                    win.set_metadata_format(
                        path.extension().map(|e| e.to_string_lossy().to_uppercase()).unwrap_or_default().into(),
                    );
                    if let Ok(meta) = std::fs::metadata(&path) {
                        win.set_metadata_file_size(format!("{} bytes", meta.len()).into());
                        win.set_metadata_created(
                            meta.created().ok().map(|t| format!("{:?}", t)).unwrap_or_default().into(),
                        );
                        win.set_metadata_modified(
                            meta.modified().ok().map(|t| format!("{:?}", t)).unwrap_or_default().into(),
                        );
                    }
                    win.set_status_text(format!("{} × {} px", width, height).into());
                    win.set_zoom(1.0);
                    win.set_pan_x(0.0);
                    win.set_pan_y(0.0);
                });
            }
            Err(e) => {
                if req_id != REQUEST_COUNTER.load(Ordering::SeqCst) {
                    return;
                }
                let error_msg = format!("Failed to load image: {}", e);
                let _ = weak_window.upgrade_in_event_loop(move |win| {
                    if req_id != REQUEST_COUNTER.load(Ordering::SeqCst) {
                        return;
                    }
                    win.set_has_image(false);
                    win.set_error_text(error_msg.into());
                    win.set_status_text("Error loading image".into());
                });
            }
        }
    });
}

fn open_and_populate_file(file: PathBuf, weak: Weak<MainWindow>, state: Arc<Mutex<AppState>>) {
    if let Some(parent) = file.parent() {
        if let Ok(media_files) = filesystem::scan_directory(parent) {
            let image_files: Vec<PathBuf> = media_files
                .into_iter()
                .filter(|f| f.is_image)
                .map(|f| f.path)
                .collect();

            let target_idx = image_files.iter().position(|p| p == &file).unwrap_or(0);

            let names: Vec<slint::SharedString> = image_files
                .iter()
                .map(|p| {
                    p.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                        .into()
                })
                .collect();

            {
                let mut s = state.lock().unwrap();
                s.files = image_files;
                s.current_index = target_idx as isize;
            }

            if let Some(win) = weak.upgrade() {
                win.set_files(slint::ModelRc::new(slint::VecModel::from(names)));
                win.set_current_index(target_idx as i32);
                win.set_status_text(
                    format!("Loaded {} images", state.lock().unwrap().files.len()).into(),
                );
            }

            load_and_set_image(weak, file, state);
            return;
        }
    }

    // Fallback: single file without folder scanning
    {
        let mut s = state.lock().unwrap();
        s.files = vec![file.clone()];
        s.current_index = 0;
    }
    if let Some(win) = weak.upgrade() {
        let name: slint::SharedString = file
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
            .into();
        win.set_files(slint::ModelRc::new(slint::VecModel::from(vec![name])));
        win.set_current_index(0);
    }
    load_and_set_image(weak, file, state);
}

fn main() -> anyhow::Result<()> {
    let main_window = MainWindow::new()?;
    let state = Arc::new(Mutex::new(AppState::new()));
    let weak = main_window.as_weak();

    // 1. Open File
    {
        let state = state.clone();
        let weak = weak.clone();
        main_window.on_open_file(move || {
            if let Some(file) = rfd::FileDialog::new()
                .add_filter(
                    "Image files",
                    &["png", "jpg", "jpeg", "webp", "gif", "bmp", "tiff", "tif"],
                )
                .pick_file()
            {
                open_and_populate_file(file, weak.clone(), state.clone());
            }
        });
    }

    // 2. Open Folder
    {
        let state = state.clone();
        let weak = weak.clone();
        main_window.on_open_folder(move || {
            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                match filesystem::scan_directory(&folder) {
                    Ok(media_files) => {
                        let image_files: Vec<PathBuf> = media_files
                            .into_iter()
                            .filter(|f| f.is_image)
                            .map(|f| f.path)
                            .collect();
                        let names: Vec<slint::SharedString> = image_files
                            .iter()
                            .map(|p| {
                                p.file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string()
                                    .into()
                            })
                            .collect();
                        {
                            let mut s = state.lock().unwrap();
                            s.files = image_files;
                            s.current_index = if s.files.is_empty() { -1 } else { 0 };
                        }
                        if let Some(win) = weak.upgrade() {
                            win.set_files(slint::ModelRc::new(slint::VecModel::from(names)));
                            win.set_current_index(0);
                            win.set_status_text(
                                format!("Loaded {} images", state.lock().unwrap().files.len())
                                    .into(),
                            );
                            if let Some(path) = state.lock().unwrap().current_path().cloned() {
                                load_and_set_image(weak.clone(), path, state.clone());
                            }
                        }
                    }
                    Err(e) => {
                        if let Some(win) = weak.upgrade() {
                            win.set_status_text(format!("Scan failed: {}", e).into());
                        }
                    }
                }
            }
        });
    }

    // 3. New Window
    main_window.on_new_window(move || {
        if let Ok(exe) = std::env::current_exe() {
            let _ = std::process::Command::new(exe).spawn();
        }
    });

    // 4. Quit
    main_window.on_quit_app(move || {
        std::process::exit(0);
    });

    // 5. Set as Background (GNOME)
    {
        let state = state.clone();
        main_window.on_set_wallpaper(move || {
            if let Some(path) = state.lock().unwrap().current_path().cloned() {
                let uri = format!("file://{}", path.to_string_lossy());
                let _ = std::process::Command::new("gsettings")
                    .args(["set", "org.gnome.desktop.background", "picture-uri", &uri])
                    .spawn();
                let _ = std::process::Command::new("gsettings")
                    .args([
                        "set",
                        "org.gnome.desktop.background",
                        "picture-uri-dark",
                        &uri,
                    ])
                    .spawn();
            }
        });
    }

    // 6. Select File from list
    {
        let state = state.clone();
        let weak = weak.clone();
        main_window.on_select_file(move |idx| {
            {
                let mut s = state.lock().unwrap();
                if idx >= 0 && (idx as usize) < s.files.len() {
                    s.current_index = idx as isize;
                }
            }
            if let Some(path) = state.lock().unwrap().current_path().cloned() {
                if let Some(win) = weak.upgrade() {
                    win.set_current_index(idx);
                }
                load_and_set_image(weak.clone(), path, state.clone());
            }
        });
    }

    // 7. Zoom Controls
    {
        let weak = weak.clone();
        main_window.on_zoom_in(move || {
            if let Some(win) = weak.upgrade() {
                let z = (win.get_zoom() * 1.25).min(10.0);
                win.set_zoom(z);
            }
        });
    }
    {
        let weak = weak.clone();
        main_window.on_zoom_out(move || {
            if let Some(win) = weak.upgrade() {
                let z = (win.get_zoom() / 1.25).max(0.1);
                win.set_zoom(z);
            }
        });
    }
    {
        let weak = weak.clone();
        main_window.on_fit_window(move || {
            if let Some(win) = weak.upgrade() {
                win.set_zoom(1.0);
                win.set_pan_x(0.0);
                win.set_pan_y(0.0);
            }
        });
    }
    {
        let state = state.clone();
        let weak = weak.clone();
        main_window.on_actual_size(move || {
            if let Some(win) = weak.upgrade() {
                let s = state.lock().unwrap();
                if s.original_width > 0 {
                    let approx_zoom = (s.original_width as f32 / 800.0).max(0.1).min(8.0);
                    win.set_zoom(approx_zoom);
                } else {
                    win.set_zoom(1.0);
                }
                win.set_pan_x(0.0);
                win.set_pan_y(0.0);
            }
        });
    }

    // 8. Rotation Controls
    {
        let state = state.clone();
        let weak = weak.clone();
        main_window.on_rotate_left(move || {
            let mut s = state.lock().unwrap();
            s.rotation = (s.rotation - 90).rem_euclid(360);
            if let Some(win) = weak.upgrade() {
                win.set_rotation(s.rotation);
            }
        });
    }
    {
        let state = state.clone();
        let weak = weak.clone();
        main_window.on_rotate_right(move || {
            let mut s = state.lock().unwrap();
            s.rotation = (s.rotation + 90).rem_euclid(360);
            if let Some(win) = weak.upgrade() {
                win.set_rotation(s.rotation);
            }
        });
    }

    // 9. Navigation Controls
    {
        let state = state.clone();
        let weak = weak.clone();
        main_window.on_prev_image(move || {
            let path = {
                let mut s = state.lock().unwrap();
                if s.current_index > 0 {
                    s.current_index -= 1;
                    s.current_path().cloned()
                } else {
                    None
                }
            };
            if let Some(path) = path {
                if let Some(win) = weak.upgrade() {
                    win.set_current_index(state.lock().unwrap().current_index as i32);
                }
                load_and_set_image(weak.clone(), path, state.clone());
            }
        });
    }
    {
        let state = state.clone();
        let weak = weak.clone();
        main_window.on_next_image(move || {
            let path = {
                let mut s = state.lock().unwrap();
                let len = s.files.len() as isize;
                if s.current_index >= 0 && s.current_index + 1 < len {
                    s.current_index += 1;
                    s.current_path().cloned()
                } else {
                    None
                }
            };
            if let Some(path) = path {
                if let Some(win) = weak.upgrade() {
                    win.set_current_index(state.lock().unwrap().current_index as i32);
                }
                load_and_set_image(weak.clone(), path, state.clone());
            }
        });
    }

    // 10. Native Fullscreen Toggle
    {
        let weak = weak.clone();
        main_window.on_toggle_fullscreen(move || {
            if let Some(win) = weak.upgrade() {
                let is_fs = win.window().is_fullscreen();
                win.window().set_fullscreen(!is_fs);
                win.set_is_fullscreen(!is_fs);
            }
        });
    }

    // CLI Argument Handling: Open file or folder if provided on command line
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(first_arg) = args.first() {
        let path = PathBuf::from(first_arg);
        if path.exists() {
            if path.is_dir() {
                if let Ok(media_files) = filesystem::scan_directory(&path) {
                    let image_files: Vec<PathBuf> = media_files
                        .into_iter()
                        .filter(|f| f.is_image)
                        .map(|f| f.path)
                        .collect();
                    let names: Vec<slint::SharedString> = image_files
                        .iter()
                        .map(|p| {
                            p.file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string()
                                .into()
                        })
                        .collect();
                    {
                        let mut s = state.lock().unwrap();
                        s.files = image_files;
                        s.current_index = if s.files.is_empty() { -1 } else { 0 };
                    }
                    main_window.set_files(slint::ModelRc::new(slint::VecModel::from(names)));
                    main_window.set_current_index(0);
                    if let Some(first_img) = state.lock().unwrap().current_path().cloned() {
                        load_and_set_image(weak.clone(), first_img, state.clone());
                    }
                }
            } else if path.is_file() {
                open_and_populate_file(path, weak.clone(), state.clone());
            }
        }
    }

    main_window.run()?;
    Ok(())
}
