use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
    let mut sheet_1: Vec<i64> = Vec::new();
    let mut sheet_2: Vec<i64> = Vec::new();


    for line in buf_reader.lines() {
        let content = line.expect("oops file error");


        let parts_of_content: Vec<&str> = content.split("   ").collect();

        let num_1 = parts_of_content[0].parse::<i64>().unwrap();
        let num_2 = parts_of_content[1].parse::<i64>().unwrap();
        sheet_1.push(num_1);
        sheet_2.push(num_2);


    }
   
        
    sheet_1.sort();
    sheet_2.sort();

    let mut diff: i64 = 0;


    for (count, number) in sheet_1.iter().enumerate(){
        /* 
        println!("position {}", count);
        println!("sheet 1 {}", sheet_1[count]);
        println!("sheet 2 {}", sheet_2[count]);
        */

        let tmp_diff: i64 = (sheet_1[count] - sheet_2[count]).abs();
        diff += tmp_diff;
    }

    /* 
    println!("Sheet 1");
    for nu in &sheet_1{
        print!("{} ", nu);
    }
    println!("Sheet 2");
    for nu in &sheet_2{
        print!("{} ", nu);
    }
    */
    println!("done with diff {}", diff);
    Ok(())

}
