use std::fs::File;
use std::io::{self, BufRead};

pub fn compare(file1:&File, file2:&File)  {
    
    let f1 = io::BufReader::new(file1).lines();
    let f2 = io::BufReader::new(file2).lines();

    let mut f1_iter = f1.enumerate();
    let mut f2_iter = f2.enumerate();

    loop {
        let f1_line = f1_iter.next();
        let f2_line = f2_iter.next();
        
        match (f1_line, f2_line) {
            (Some((i,Ok(line1))),
             Some((j,Ok(line2)))) => 
                modified_lines(i+1,&line1,j+1,&line2),

            (Some((i,Ok(line))),
             None) => 
                removed_line(i+1,&line),

            (None,
             Some((i,Ok(line)))) =>
                added_line(i+1,&line),

            (None,None) => break,
            _ => break,
        }
    }
}


fn modified_lines(index1:usize, line1:&str, index2:usize, line2:&str) {
    if line1 != line2 {
        println!("file 1:\t{}\t| - {}\nfile 2:\t{}\t| + {}"
        , index1,line1, index2,line2);
    }
}
fn added_line(index:usize, line:&str) {
    if line.is_empty() {return;}
    println!("\t{}\t| + {}", index,line);
}
fn removed_line(index:usize, line:&str) {
    if line.is_empty() {return;}
    println!("\t{}\t| - {}", index,line);
}
