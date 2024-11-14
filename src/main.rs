mod loader;
mod comparator;

fn main() {

    let args:Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Enter two files");
        return;
    }

    let file1 = loader::get_file(&args[1]);
    let file2 = loader::get_file(&args[2]);
    

    if !loader::verify_files(&file1, &file2) {
        eprintln!("File not found");
        return;
    }    
    
    comparator::compare(&file1.unwrap() ,&file2.unwrap());
    
}
