
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::usize;
use std::time::Instant;

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let start = Instant::now();
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let mut output: u64=0;
    
    

    for(y, line) in buf_reader.lines().enumerate() {
        let content = line.expect("oops file error");
        let mut new_Stones: Vec<u64> = content.split(" ").map(|F| F.parse::<u64>().expect("failed to parse split to digit")).collect();

        let mut answer_hash = HashMap::new();
        for stone in new_Stones.iter(){
            
            output+=do_stone(stone.clone(), 75, &mut answer_hash);
        }       
    }
    
    let duration = start.elapsed();
    println!("done: {} in {:?}", output,duration);

    Ok(())

}


fn do_stone(stone: u64, count: usize,answer_cache: &mut HashMap<(u64, usize), u64>)->u64{

    if count==0{
        return 1;
    }
    let key = (stone, count);
    if let Some(answer_cache)= answer_cache.get(&key){
        return *answer_cache;
    }
    let string_of_stone = stone.to_string();
    if stone == 0{
        return do_stone(1 as u64, count -1, answer_cache);
        
    }else if string_of_stone.len() %2==0{
        let (bit_1,bit_2) = string_of_stone.split_at(string_of_stone.len()/2);
        return do_stone(bit_1.parse::<u64>().expect("number expected"), count -1, answer_cache)+ do_stone(bit_2.parse::<u64>().expect("number expected"), count -1, answer_cache);
       
        
    }
    let res = do_stone(stone * 2024, count -1, answer_cache);
    answer_cache.insert(key, res);
    res

    
    
}




