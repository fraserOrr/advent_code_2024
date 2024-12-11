
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use cached::proc_macro::cached;
use fn_cache::{FnCache, HashCache};


#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct small_stone{
    value: i64,
    blink: i64,
}
fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let mut output: usize=0;
    let mut starting_stones: Vec<i64> = Vec::new();
    let mut stones: Vec<i64> = Vec::new();

    for(y, line) in buf_reader.lines().enumerate() {
        let content = line.expect("oops file error");
        let mut new_Stones: Vec<i64> = content.split(" ").map(|F| F.parse::<i64>().expect("failed to parse split to digit")).collect();
        stones.append(&mut new_Stones);

        
    }

  
   
     //make some small jumps
     /*  */
    let mut output_holder: Vec<Vec<i64>>= Vec::new();
    for j in 0..10{
        let mut test_stones: Vec<i64> = Vec::new();
        let mut test_stones_2: Vec<i64> = Vec::new();
        test_stones.push(j as i64);
        
        for i in 0..25{
            println!("small Run: {}.{}", j,i);
            test_stones = calc_next_blink(test_stones);
            
            test_stones_2.push(test_stones.len() as i64);
            //println!("Stones: {:?}", test_stones);
        }
        drop(test_stones);
        output_holder.push(test_stones_2);
    }

    for (k,test_output) in output_holder.iter().enumerate(){
        println!("for input {} the lengths after 25 goes are {:?}",k, test_output)
    }
   
    let blinks: i64 = 25;



    for i in 0..blinks{
        println!("small Run: {}", i);
        stones = calc_next_blink( stones);
        
       
        //println!("Stones: {:?}", test_stones);
    }


    
    println!("done: {}", stones.len());
   
    
    
    Ok(())

}



fn do_stone(stone: i64)->Vec<i64>{
    let mut return_stuff: Vec<i64> = Vec::new();
    let mut curr_stone = stone;
    let string_of_stone = curr_stone.to_string();
    if curr_stone == 0{
        curr_stone=1;
        return_stuff.push(curr_stone); 
        
    }else if string_of_stone.len() %2==0{
        let (bit_1,bit_2) = string_of_stone.split_at(string_of_stone.len()/2);
        
        return_stuff.push(bit_1.parse::<i64>().expect("number expected"));
        return_stuff.push(bit_2.parse::<i64>().expect("number expected"));
        
    }else{

        curr_stone*=2024;
        return_stuff.push(curr_stone); 

    }
    return  return_stuff;
}

fn calc_next_blink(stones:  Vec<i64>)->Vec<i64>{
    let mut i: usize = 0;
    let mut return_stuff: Vec<i64> = Vec::new();
    for stone in stones.iter() {

        return_stuff.append(&mut do_stone(stone.clone()));
        i+=1;
        
    }

    return return_stuff;
}


