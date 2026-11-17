mod ui;
use rune::{AppSpec, ImageCache, Pixels, WindowOptions};
use std::path::PathBuf;

pub(crate) struct State {
    pub(crate) files: Vec<PathBuf>, pub(crate) current: isize, pub(crate) zoom: f32, pub(crate) rotation: i32,
    pub(crate) pan_x: f32, pub(crate) pan_y: f32, pub(crate) drag_pan_x: f32, pub(crate) drag_pan_y: f32,
    pub(crate) image: Pixels, pub(crate) icons: ui::icons::Icons, pub(crate) cache: Option<ImageCache>, pub(crate) has_image: bool,
    pub(crate) error: String, pub(crate) filename: String, pub(crate) dimensions: String, pub(crate) image_width: u32, pub(crate) image_height: u32, pub(crate) folder: String, pub(crate) format: String, pub(crate) file_size: String,
    pub(crate) show_info: bool, pub(crate) zoom_menu: bool, pub(crate) show_menu: bool,
}
impl Default for State { fn default()->Self{Self{files:vec![],current:-1,zoom:1.0,rotation:0,pan_x:0.0,pan_y:0.0,drag_pan_x:0.0,drag_pan_y:0.0,image:Pixels::new(),icons:Default::default(),cache:None,has_image:false,error:String::new(),filename:String::new(),dimensions:String::new(),image_width:0,image_height:0,folder:String::new(),format:String::new(),file_size:String::new(),show_info:false,zoom_menu:false,show_menu:false}} }

fn format_size(bytes:u64)->String{if bytes>=1024*1024{format!("{:.1} MB",bytes as f64/(1024.0*1024.0))}else{format!("{:.1} KB",bytes as f64/1024.0)}}
fn load(s:&mut State,path:PathBuf){match image_engine::decode_to_rgba(&path){Ok(d)=>{s.image.set(d.width,d.height,d.rgba);s.has_image=true;s.error.clear();s.zoom=1.0;s.rotation=0;s.pan_x=0.0;s.pan_y=0.0;s.image_width=d.width;s.image_height=d.height;s.filename=path.file_name().unwrap_or_default().to_string_lossy().into();s.dimensions=format!("{} × {}",d.width,d.height);s.folder=path.parent().map(|p|p.display().to_string()).unwrap_or_default();s.format=path.extension().map(|e|e.to_string_lossy().to_uppercase()).unwrap_or_default();s.file_size=std::fs::metadata(path).map(|m|format_size(m.len())).unwrap_or_default();},Err(e)=>{s.has_image=false;s.error=format!("Failed to load image: {e}");}}}
pub(crate) fn open_file(s:&mut State){if let Some(path)=rfd::FileDialog::new().add_filter("Image files", &["png","jpg","jpeg","webp","gif","bmp","tiff","tif"]).pick_file(){if let Some(parent)=path.parent(){if let Ok(media)=filesystem::scan_directory(parent){s.files=media.into_iter().filter(|m|m.is_image).map(|m|m.path).collect();s.current=s.files.iter().position(|p|p==&path).unwrap_or(0)as isize;}}load(s,path);}}
pub(crate) fn open_folder(s:&mut State){if let Some(folder)=rfd::FileDialog::new().pick_folder(){if let Ok(media)=filesystem::scan_directory(&folder){s.files=media.into_iter().filter(|m|m.is_image).map(|m|m.path).collect();s.current=if s.files.is_empty(){-1}else{0};if let Some(path)=s.files.first().cloned(){load(s,path);}}}}
pub(crate) fn navigate(s:&mut State,d:isize){let n=s.current+d;if n>=0&&(n as usize)<s.files.len(){s.current=n;if let Some(p)=s.files.get(n as usize).cloned(){load(s,p);}}}

fn main(){let mut spec=AppSpec::new("Veyra",(680.0,460.0),State::default,ui::view);spec.setup=Some(|s,w|{let c=s.cache.insert(ImageCache::new(w));s.icons=ui::icons::load(c);});spec.poll=Some(ui::sync_title);rune::App::new(spec).window_options(WindowOptions{borderless:true,transparent:false,resizable:true,min_size:Some((480.0,360.0)),..Default::default()}).run();}
