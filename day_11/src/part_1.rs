use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::ops::Index;




fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let mut output: usize=0;
    let mut stones: Vec<i64> = Vec::new();

    for(y, line) in buf_reader.lines().enumerate() {
        let content = line.expect("oops file error");
        let mut new_Stones: Vec<i64> = content.split(" ").map(|F| F.parse::<i64>().expect("failed to parse split to digit")).collect();
        stones.append(&mut new_Stones);

        
    }
    println!("Stones: {:?}", stones);
    let blinks: i64 = 25;

    for i in 0..blinks{
        //println!("Run: {}", i);
        stones = calc_next_blink(&mut stones);
        //println!("Stones: {:?}", stones);
    }



    

    println!("done : {}", stones.len());
    
    
    Ok(())

}


fn calc_next_blink(stones:  &mut Vec<i64>)->Vec<i64>{
    let mut i: usize = 0;
    while i < stones.len() {
        let mut curr_stone = stones[i];
        let mut string_of_stone = curr_stone.to_string();
        if curr_stone == 0{
            curr_stone=1;
            stones[i] = curr_stone;
            
        }else if string_of_stone.len() %2==0{
            let(bit_1,bit_2) = string_of_stone.split_at(string_of_stone.len()/2);
            
            stones.remove(i);
            stones.insert(i, bit_2.parse::<i64>().expect("number expected"));
            stones.insert(i, bit_1.parse::<i64>().expect("number expected"));
            i+=1;
        }else{

            curr_stone*=2024;
            stones[i] = curr_stone;

        }
        i+=1;
        
    }

    return stones.clone();
}