use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;



fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let output: i64 =0;
    for line in buf_reader.lines() {
        let content = line.expect("oops file error");


    }

    
    
    println!("done : {}", output);
    

    
    Ok(())

}
