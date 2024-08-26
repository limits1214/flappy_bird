use super::{Ffi, FfiKv};
use directories::ProjectDirs;
use std::{
    io::{Read, Write},
    path::PathBuf,
};

const QUALIFIER: &str = "qualifier";
const ORGANIZATION: &str = "organization";
const APPLICATION: &str = "bevy_flappybird";
const KV_DIR: &str = "kv";

impl FfiKv for Ffi {
    fn get(key: &str) -> String {
        if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            let path = proj_dirs.data_dir();
            let mut file_path = PathBuf::from(path);
            file_path.push(KV_DIR);
            file_path.push(key);

            let mut file = match std::fs::File::open(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    println!("file open fail err: {}", e);
                    return String::new();
                }
            };

            let mut contents = String::new();
            if let Err(e) = file.read_to_string(&mut contents) {
                println!("file read fail err: {}", e);
                return String::new();
            }

            contents
        } else {
            println!("proj dir fail");
            String::new()
        }
    }

    fn set(key: &str, val: &str) {
        if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            let path = proj_dirs.data_dir();
            let mut file_path = PathBuf::from(path);
            file_path.push(KV_DIR);

            if let Err(e) = std::fs::create_dir_all(&file_path) {
                println!("dir create fail: {}", e);
                return;
            }

            file_path.push(format!("{key}"));

            let mut file = match std::fs::File::create(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    println!("file create fail err: {}", e);
                    return;
                }
            };

            if let Err(e) = file.write_all(val.as_bytes()) {
                println!("file write fail err: {}", e);
                return;
            }
        } else {
            println!("proj dir fail");
        }
    }
}

#[test]
fn get_test() {
    use super::Score;
    let get = Ffi::get("test");
    let score = serde_json::from_str::<Score>(&get).unwrap();
    println!("get: {:?}", score);
}

#[test]
fn set_test() {
    use super::Score;
    let score = Score { score: 1200 };
    let testjson = serde_json::to_string(&score).unwrap();
    Ffi::set("test", &testjson);
}
