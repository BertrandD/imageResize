extern crate image;

use std::path::Path;
use image::GenericImage;

static mut RECURSIVE: bool = false;
static mut WIDTH: u32 = 600;

fn process_image(path : &Path) {

    if path.file_name().unwrap().to_str().unwrap().contains("modified_") {
        return;
    }

    let img = image::open(&path).unwrap();

    let w = img.width();
    let h = img.height();
    unsafe {
        let nw = WIDTH;
        let nh = nw * h / w;

        let ref mut new_image = image::imageops::resize(&img, nw, nh, image::FilterType::CatmullRom);

        println!("input  : {:?}", path);
        //    println!("dimensions {:?}", img.dimensions());

        let mut new_path = std::path::PathBuf::from(path.parent().unwrap());
        new_path.push(format!("modified_{}_{}x{}.{}",path.file_name().unwrap().to_str().unwrap(), nw, nh, path.extension().unwrap().to_str().unwrap()));

        println!("output : {:?}", new_path.as_path());

//        std::fs::create_dir(&new_path.as_path().parent().unwrap());

        image::save_buffer(&new_path.as_path(), new_image, nw, nh, img.color()).unwrap();
    }
}

fn display_help() {
    println!("usage: resize [--help] [-r] [-w WIDTH] folder");
    println!("Arguments :");
    println!("\t-h --help: print this help");
    println!("\t-r: Recursive resize image");
    println!("\t-w: width of modified image (default: 600)");
}

fn process_dir(arg: &Path) {
    if arg.is_dir() {
        for entry in std::fs::read_dir(arg).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                unsafe {
                    if RECURSIVE {
                        process_dir(&path)
                    }
                }
            } else {
                process_image(&path);
            }
        }
    }
}

fn process_argument(arg : String) {
    let path = Path::new(&arg);

    if path.is_dir() {
        process_dir(path);
    } else {
        process_image(path);
    }
//    println!("Unknown argument : {}", arg)
}

fn enable_recursive() {
    unsafe {
        RECURSIVE = true;
    }
}

fn set_width(w : String){
    let wi = w.parse::<u32>().unwrap();
    unsafe {
        WIDTH = wi;
    }
}

fn main() {

    let mut args = std::env::args();

    args.next();
    while args.size_hint().0 > 0 {
        let arg = args.next().unwrap();
        match arg.as_ref() {
            "--help" => display_help(),
            "-h" => display_help(),
            "-r" => enable_recursive(),
            "-w" => set_width(args.next().unwrap()),
            _ => process_argument(arg)
        }
    }
}