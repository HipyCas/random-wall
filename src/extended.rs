use std::{path::{Path, PathBuf}, fs, marker::{PhantomData}, env};
use rocket::{serde::json::Json, fs::NamedFile};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref WALLPAPERS: WallpaperStore<'static> = WallpaperStore::new(env::var("WALLPAPER_FOLDER").expect("Wallpaper folder not set"));
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![random_wall, wallpaper, full_path])
}

#[get("/")]
fn random_wall() -> Json<String> {
    Json(uri!(full_path(WALLPAPERS.random_src())).to_string()) // You can make this only return a simple &'static str
}

#[get("/wall/<path..>")]
async fn wallpaper(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(PathBuf::from(env::var("WALLPAPER_FOLDER").expect("variable not set")).join(path)).await.ok()
}

#[get("/p/<path..>")]
async fn full_path(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("/").join(path)).await.ok()
}

#[derive(Debug, Serialize)]
struct Wallpaper {
    source: PathBuf
}

impl Wallpaper {
    pub fn as_ref(&self) -> BorrowedWallpaper {
        BorrowedWallpaper { source: self.source.as_path()
         }
    }
}

#[derive(Debug, Serialize)]
struct BorrowedWallpaper<'a> {
    source: &'a Path
}

struct WallpaperStore<'a> {
    wallpapers: Vec<Wallpaper>,
    phantom: PhantomData<&'a str>
}

impl<'a> WallpaperStore<'a> {
    pub fn new<P>(path:P ) -> WallpaperStore<'a> where P: AsRef<Path> {
        WallpaperStore { wallpapers: 
            fs::read_dir(path).expect("Could not read target dir").map(|item| item.expect("Could not read item")).map(|file| Wallpaper {
                source: file.path()
            }).collect(),
            phantom: PhantomData::default()
         }
    }

    pub fn random(&self) -> BorrowedWallpaper {
        let rnd = f64::floor(rand::random::<f64>() * (self.wallpapers.len() as f64)) as usize;
        self.wallpapers.get(rnd).expect(&format!("Cannot get wallpaper at position {rnd} ({})", self.wallpapers.len())).as_ref()
    }

    pub fn random_src(&'a self) -> &'a Path {
        self.random().source
    }
}