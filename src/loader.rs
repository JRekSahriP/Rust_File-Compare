use std::fs::File;
use std::io;

pub fn get_file(path:&str) -> Result<File, io::Error> {
    File::open(path)
}

pub fn verify_files(f1:&Result<File, io::Error>, f2:&Result<File, io::Error>) -> bool {
    let mut ok = true;
    if let Err(_) = f1 {
        eprintln!("Error to read first file");
        ok = false;
    } 
    if let Err(_) = f2 {
        eprintln!("Error to read second file");
        ok = false;
    }

    ok
}