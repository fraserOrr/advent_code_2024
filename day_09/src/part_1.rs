use core::num;
use std::collections::binary_heap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Display;
use std::fmt;

#[derive(Clone, Copy ,Debug)]
enum Bit {
    File { file_id: i64},
    Blank {character: char},
}




fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let mut output: usize = 0;
    let mut uncompressed_format: Vec<Bit> = Vec::new();


    for line in buf_reader.lines() {
        let content = line.expect("oops file error");
        let input_cotainer: Vec<i64> = content.chars().map(|x| x.to_digit(10).expect("failed to conver to digit") as i64 ).collect();
        println!("Puzzle input {:?}", input_cotainer);
        for i in 0..input_cotainer.len(){
            if i % 2 == 0{
                //on a file bit
                let mut file_id: i64 = 0;
                if i != 0{
                    file_id= i as i64 / 2;

                }
                //println!("file {} has length {}", file_id, input_cotainer[i]);
                for j in 0..input_cotainer[i]{
                    let tmp: Bit = Bit::File { file_id: file_id };
                    uncompressed_format.push(tmp);
                }
            }else{
                // on a blank space bit
                //println!("found blank space {}", input_cotainer[i]);
                for j in 0..input_cotainer[i]{
                    let tmp: Bit = Bit::Blank { character: '.' };
                    uncompressed_format.push(tmp);
                }
                
            }
        }
    }

    println!();
    for bit in uncompressed_format.iter(){
        inspect(bit);
        
    }
    println!();
    println!("len {}", uncompressed_format.len());
    
    //make a compressed version

    for i in (0..uncompressed_format.len()).rev(){
        
        if what_type(&uncompressed_format[i])==false{
            
            //on a empty item
            //go get the last item in bring it here
            for j in 0..i{
                if what_type(&uncompressed_format[j])==true{
                    // now to swap them over?
                    
                    let item_tmp = uncompressed_format[i];
                    //println!("Swapping {:?} to {}",item_tmp, j );
                    uncompressed_format[j] = item_tmp;
                    uncompressed_format[i] = Bit::Blank { character: '.' };
                    break;
                }
            }
        }    
    }
    //check 
    for bit in uncompressed_format.iter(){
        inspect(bit);
        
    }
    println!();
    println!("len {}", uncompressed_format.len());

    //make checksome
    for (i,bit ) in uncompressed_format.iter().enumerate(){
        if what_type(bit)==false{
            output+= i * get_nu(bit);
        }
        
    }
    println!("done : {}", output);
    
    
    Ok(())

}

fn inspect(item: &Bit){
    match  item {
        Bit::Blank { character } => print!("{}", character.to_string()),
        Bit::File { file_id } =>print!("'{}'", file_id.to_string()),

    }
}
fn what_type(item: &Bit)-> bool{
    match  item {
        Bit::Blank { character } => true,
        Bit::File { file_id } => false,

    }
}
fn get_nu(item: &Bit)-> usize{
    match  item {
        Bit::Blank { character } => panic!("blank has no number"),
        Bit::File { file_id } => file_id.clone() as usize,

    }
}