use core::num;
use std::collections::binary_heap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Display;
use std::fmt;

                        
#[derive(Clone, Copy,PartialEq ,Debug)]
enum Bit {
    FileDisk {item: FileDisk} ,
    EmptyDisk {item: EmptyDisk},
}
#[derive(Clone, Copy,PartialEq ,Debug)]
struct FileDisk{
    file_id: i64,
    size: i64,
    start: i64,
}
#[derive(Clone, Copy,PartialEq ,Debug)]
struct EmptyDisk{
    character: char,
    size: i64,
    start: i64,
}

fn inspect(item: &Bit){
    match  item {
        Bit::EmptyDisk { item }=> print!("{}", item.character.to_string()),
        Bit::FileDisk { item } =>print!("'{}'", item.file_id.to_string()),

    }
}
fn full_print(item: &Bit){
    match  item {
        Bit::EmptyDisk { item } => print!(" ' {}-{} ' ", item.character.to_string(), item.size),
        Bit::FileDisk { item } =>print!(" ' {}-{} ' ", item.file_id, item.size ),

    }
}

fn return_self(item: &Bit) -> Bit{
    match  item {
        Bit::EmptyDisk { item } => Bit::EmptyDisk { item: item.clone() },
        Bit::FileDisk { item }=> Bit::FileDisk { item: item.clone()  },

    }
}

fn what_type(item: &Bit)-> bool{
    match  item {
        Bit::EmptyDisk { item } => true,
        Bit::FileDisk { item }=> false,

    }
}

fn get_size(item: &Bit)-> i64{
    match  item {
        Bit::EmptyDisk { item } => item.size,
        Bit::FileDisk { item }=> item.size,

    }
}
fn get_start(item: &Bit)-> i64{
    match  item {
        Bit::EmptyDisk { item } => item.start,
        Bit::FileDisk { item }=> item.start,

    }
}
fn get_nu(item: &Bit)-> i64{
    match  item {
        Bit::EmptyDisk { item } => panic!("blank has no number"),
        Bit::FileDisk { item } => item.file_id.clone() as i64,

    }
}

fn big_print(bit_container: &Vec<Bit>){
    println!();

  
    for bit in bit_container.iter(){
        if what_type(&bit)==false{
            let file_id = get_nu(bit);
            
            for i in 0..get_size(bit){              
                print!("{}", file_id)
            }
        }else{
            for i in 0..get_size(bit){
                print!("{}", '.')
            }
        }

    }
    println!();
}

fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let mut output: i128 = 0;
    let mut uncompressed_format: Vec<Box<Bit>> = Vec::new();
    let mut bit_container: Vec<Bit> = Vec::new();

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
                    let tmp: Bit = Bit::FileDisk { item: FileDisk {file_id: file_id,size: input_cotainer[i], start: i as i64}};
                    
                    if bit_container.contains(&tmp) == false{
                        bit_container.push(tmp);
                    }
                    let index = bit_container.iter().position(|r| r == &tmp).unwrap();
                    let bit_file_assoc = Box::new(bit_container[index]);
                    uncompressed_format.push(bit_file_assoc);
                }   
            }else{
                // on a blank space bit
                //println!("found blank space {}", input_cotainer[i]);
                for j in 0..input_cotainer[i]{
                    let tmp: Bit = Bit::EmptyDisk { item: EmptyDisk { character: '.', size: input_cotainer[i], start: i as i64 } } ;
                    if bit_container.contains(&tmp) == false{
                        bit_container.push(tmp);
                    }
                    let index = bit_container.iter().position(|r| r == &tmp).unwrap();
                    let bit_file_assoc = Box::new(bit_container[index]);
                    uncompressed_format.push(bit_file_assoc);
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
    
    //put all the empty starting nodes together
    for i in 0..bit_container.len(){
        if i> 1 && i < bit_container.len(){
            if what_type(&bit_container[i])==true && what_type(&bit_container[i-1])==true{

                let size = get_size(&bit_container[i-1]) + get_size(&bit_container[i]);
                let start= get_start(&bit_container[i-1]);
                let new_empty =  Bit::EmptyDisk { item: EmptyDisk { character: '.', size: size, start: start } } ;
    
                bit_container.remove(i-1);
                bit_container.remove(i-1);
                bit_container.insert(i-1, new_empty);
                
            }
        }
        


    }

    for bit in bit_container.iter(){
        full_print(bit);
    }
    println!();

    //now we can sort  in
    let mut i: usize= bit_container.len()-1;
    while i > 0{
        // feel like i want to do something here to combine emptys
        /* 
        println!();
        for bit in bit_container.iter(){
            full_print(bit);
        }
        println!();
        big_print(&bit_container);
        println!();*/
        if what_type(&bit_container[i])==false{

            full_print(&bit_container[i]);
            //go get the last item in bring it here
            for j in 0..i{
                if what_type(&bit_container[j])==true && ( get_size(&bit_container[i]) <= get_size(&bit_container[j]) ){
                    
                    
                    
                    let size_1 = get_size(&bit_container[i]);
                    let size_2 = get_size(&bit_container[j]);
                    let size = get_size(&bit_container[j]) - get_size(&bit_container[i]);
                    let start= get_start(&bit_container[j]) + get_size(&bit_container[i]);
                    let new_empty =  Bit::EmptyDisk { item: EmptyDisk { character: '.', size: size_1, start: get_start(&bit_container[i]) } } ;

                    
                    bit_container.insert(j, bit_container[i]);
                    bit_container.remove(j+1);
                    bit_container.remove(i);
                    bit_container.insert(i, new_empty);
                    //replave the removed one with gaps
                    

                    //add new one back in 
                    if size_1 < size_2 {
                        // add the empty white space back in
                        
                        let tmp: Bit = Bit::EmptyDisk { item: EmptyDisk { character: '.', size: size, start: start } } ;

                        
                        bit_container.insert(j+1, tmp);
                        i+=1;

                    }


                    
                    break;
                }
            }
        }
        
        i-=1;  
    }

    println!();
    println!();
    for bit in bit_container.iter(){
        full_print(bit);
    }
    println!();

    let mut bit_count:i64 = 0;
    for bit in bit_container.iter(){
        if what_type(&bit)==false{
            let file_id = get_nu(bit);
            
            for i in 0..get_size(bit){
                output += bit_count as i128 *file_id as i128;
                bit_count+=1;
                print!("{}", file_id)
            }
        }else{
            for i in 0..get_size(bit){
                
                bit_count+=1;
                print!("{}", '.')
            }
        }

    }
    println!();



    println!("done : {}", output);
    
    
    Ok(())

}

