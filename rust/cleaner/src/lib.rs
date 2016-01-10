#![feature(fs_walk)]
use std::fs::walk_dir;
use std::fs;
use std::process::Command;
use std::path::Path;

macro_rules!take_or{
    ($expr:expr,$function:expr) => (match $expr {
        ::std::option::Option::Some(val) => val,
        ::std::option::Option::None => return $function,
    });

    ($expr:expr,$function:expr=>) => (match $expr {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(_)=>  return $function,
    });
}
macro_rules!take_or_ret{
        ($expr:expr) => (match $expr {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(_)=>  return,
    });
}
macro_rules!take_or_con{
        ($expr:expr) => (match $expr {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(_)=>  continue,
    });
}
macro_rules!or_con{
        ($expr:expr) => (match $expr {
        ::std::option::Option::Some(val) => val,
        ::std::option::Option::None => continue,
    });
}

fn main() {
    let read_dir = take_or_ret!(fs::read_dir("/home/psycho/RESOURCE/归档/pdf/Safari"));
    for variable in read_dir {
        let entry = take_or_con!(variable);
        let file_type = take_or_con!(entry.file_type());
        if file_type.is_dir() {
            let pathbuf = entry.path();

            if pathbuf.to_str().unwrap() == "/home/psycho/RESOURCE/归档/pdf/Safari" {
                continue;
            }
            let path_str = or_con!(pathbuf.to_str());
            let file_name = or_con!(or_con!(pathbuf.file_name()).to_str());
            compress(pathbuf.to_str().unwrap().to_string());

        }
    }

}

fn clean() {
    let walk = take_or_ret!(walk_dir("/home/psycho/RESOURCE/归档/pdf/Safari"));


    for variable in walk {
        let entry = take_or_con!(variable);
        let file_type = take_or_con!(entry.file_type());
        if file_type.is_file() {
            let pathbuf = entry.path();
            let path_str = or_con!(pathbuf.to_str());
            let file_name = or_con!(or_con!(pathbuf.file_name()).to_str());
            if file_name.ends_with(".js") || file_name.ends_with(".css") ||
               file_name == "510f1a6865" || file_name == "css" ||
               file_name.starts_with("saved_resource") {
                fs::remove_file(&pathbuf);
            }
        }
    }
}

fn compress(file_name: String) {
    println!("{:?}", file_name);
    let mut f = file_name.clone();
    f.push_str(".7z");
    let out = Command::new("7z")
                  .arg("a")
                  .arg("-r")
                  .arg(f)
                  .arg(file_name)
                  .output();

    match out {
        Ok(v) => {
            println!("stdout: {}", String::from_utf8_lossy(&v.stdout));
        }
        Err(v) => {
            println!("{:?}", v);
        }
    };
}
#[test]
fn it_works() {}
