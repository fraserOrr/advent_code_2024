use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
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

    let mut right_counter: usize = 0;
    let mut left_counter: usize = 0;
    let mut score_keeper = HashMap::new();

    while left_counter < sheet_1.len(){
        println!("L: {}, R {}", left_counter, right_counter);
        if right_counter >= sheet_2.len(){
            break;
        }

        /* 
        println!("position {}", count);
        println!("sheet 1 {}", sheet_1[count]);
        println!("sheet 2 {}", sheet_2[count]);
        */

        if sheet_1[left_counter] == sheet_2[right_counter] {
            if score_keeper.contains_key(&sheet_1[left_counter]) == false {
                let start_score: i64 = 1;
                score_keeper.insert(&sheet_1[left_counter],start_score);

                right_counter+=1;
            }else {
                let old_score = score_keeper.get(&sheet_1[left_counter]).expect("key is empty hashmap");
                score_keeper.insert(&sheet_1[left_counter], 1 + old_score);
                right_counter+=1;
            }


        }else if sheet_1[left_counter] < sheet_2[right_counter]  {
            left_counter +=1;
        }else{

            right_counter+=1;
        }

        
    }

    let mut diff: i64 = 0;
    println!("Score map");
    for (i,k) in score_keeper.iter(){
        print!("Value: {}, seen {} ", i,k);

        diff += i.clone() * k.clone();
    }
    /*
    println!("Sheet 2");
    for nu in &sheet_2{
        print!("{} ", nu);
    }
    */
    println!("done and end value is: {}", diff);
    Ok(())

}
