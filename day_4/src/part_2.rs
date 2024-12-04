use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;


struct Word {
    start_point: (usize,usize),
    direction: (i64,i64),
}

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
    let mut y_container: Vec<Vec<char>> = Vec::new();
    let mut start_points: Vec<(usize,usize)> = Vec::new();
    let mut valid_working_list: Vec<Word> = Vec::new();
    let mut output: i64 = 0;
    for (y, line) in buf_reader.lines().enumerate() {
        let content = line.expect("oops file error");
        let mut x_container: Vec<char> = Vec::new();
    
        for (x,tmp_char) in content.chars().enumerate() {
            x_container.push(tmp_char.clone());

            if tmp_char == 'A'{
               
                start_points.push((y,x));
            }
        }
               
        y_container.push(x_container);
    }

    for point in start_points.iter(){

        let y = point.0;
        let x = point.1;
        println!("curr point: {:?}", point);

        let mut count: usize = 0;
        
        //Top L =S Btm R =M
        if ( y> 0 && x>0) && ( y> 0  && x<y_container[0].len()-1) && ( y<y_container.len()-1 && x>0) && ( y<y_container.len()-1&& x<y_container[0].len()-1){
       
            if y_container[y-1][x-1]=='S' && y_container[y+1][x+1]=='M'{
                count+=1;
            }
            
            //Top L =M Btm R =S
            if y_container[y-1][x-1]=='M' && y_container[y+1][x+1]=='S'{
                count+=1;
            }

            //Top R =S Btm L =M
            if y_container[y-1][x+1]=='S' && y_container[y+1][x-1]=='M'{
                count+=1;
            }
            
            //Top R =M Btm L =S
            if y_container[y-1][x+1]=='M' && y_container[y+1][x-1]=='S'{
                count+=1;
            }

            if count>=2{
                println!("count of found: {}", count);
                output+=1;
            }
        } 
    }


    for x_con in y_container.iter(){
        println!("{:?}",x_con);
    }
    
    println!("done {}",output);
    Ok(())

}
