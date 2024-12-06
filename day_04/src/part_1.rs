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

            if tmp_char == 'X'{
                //println!("found start point at : {},{}",y,x);
                start_points.push((y,x));
            }
        }
        
        
        //println!("x container: {:?}", x_container);
        y_container.push(x_container);
    }

    for point in start_points.iter(){

        let y = point.0;
        let x = point.1;
        println!("curr point: {:?}", point);
        //check up
        if y >= 3 {
            if y_container[y-1][x] == 'M' {
                let tmp_word = Word{
                    start_point: point.clone(),
                    direction: (0,-1),
                };

                valid_working_list.push(tmp_word);
            }
        }
        //check down
        if y<y_container.len()-3{
            if y_container[y+1][x] == 'M' {
                let tmp_word = Word{
                    start_point: point.clone(),
                    direction: (0,1),
                };

                valid_working_list.push(tmp_word);
            }
        }
        
        //check left
        if x >= 3 {
            if y_container[y][x-1] == 'M' {
                let tmp_word = Word{
                    start_point: point.clone(),
                    direction: (-1,0),
                };

                valid_working_list.push(tmp_word);
            }
        }
        //check right
        if x<y_container[0].len()-3{
            if y_container[y][x+1] == 'M' {
                let tmp_word = Word{
                    start_point: point.clone(),
                    direction: (1,0),
                };

                valid_working_list.push(tmp_word);
            }
        }
        //check down-left
        if y<y_container.len()-3 &&  x >= 3{
            if y_container[y+1][x-1] == 'M' {
                let tmp_word = Word{
                    start_point: point.clone(),
                    direction: (-1,1),
                };

                valid_working_list.push(tmp_word);
            }
        }
        //check down-right
        if y<y_container.len()-3 &&  x<y_container[0].len()-3{
            if y_container[y+1][x+1] == 'M' {
                let tmp_word = Word{
                    start_point: point.clone(),
                    direction: (1,1),
                };

                valid_working_list.push(tmp_word);
            }
        }
        
        //check up-left
        if y >=3 && x >= 3 {
            if y_container[y-1][x-1] == 'M' {
                let tmp_word = Word{
                    start_point: point.clone(),
                    direction: (-1,-1),
                };

                valid_working_list.push(tmp_word);
            }
        }
        //check up-right
        if y >= 3 && x<y_container[0].len()-3{
            if y_container[y-1][x+1] == 'M' {
                let tmp_word = Word{
                    start_point: point.clone(),
                    direction: (1,-1),
                };

                valid_working_list.push(tmp_word);
            }
        }

    }


    for x_con in y_container.iter(){
        println!("{:?}",x_con);
    }
    for cur_word in valid_working_list.iter(){
        let y = cur_word.start_point.0;
        let x = cur_word.start_point.1;
        
        if cur_word.direction==(0,-1){
            //up
            if y_container[y-2][x] == 'A' && y_container[y-3][x] == 'S' {
                println!("Found word at {},{} going up",y,x);
                output+=1;
            }

        }else if cur_word.direction==(0,1){
            //down
            if y_container[y+2][x] == 'A' && y_container[y+3][x] == 'S' {
                println!("Found word at {},{} going down",y,x);
                output+=1;
            }

        }else if cur_word.direction==(-1,0){
            //left
            if y_container[y][x-2] == 'A' && y_container[y][x-3] == 'S' {
                println!("Found word at {},{} going left",y,x);
                output+=1;
            }

        }else if cur_word.direction==(1,0){
            //right
            if y_container[y][x+2] == 'A' && y_container[y][x+3] == 'S' {
                println!("Found word at {},{} going right",y,x);
                output+=1;
            }

        }else if cur_word.direction==(1,-1){
            //up-right
            if y_container[y-2][x+2] == 'A' && y_container[y-3][x+3] == 'S' {
                println!("Found word at {},{} going up-right",y,x);
                output+=1;
            }

        }else if cur_word.direction==(-1,-1){
            //up-left
            if y_container[y-2][x-2] == 'A' && y_container[y-3][x-3] == 'S' {
                println!("Found word at {},{} going up-left",y,x);
                output+=1;
            }

        }else if cur_word.direction==(1,1){
            //down-right
            if y_container[y+2][x+2] == 'A' && y_container[y+3][x+3] == 'S' {
                println!("Found word at {},{} going down-right",y,x);
                output+=1;
            }

        }else if cur_word.direction==(-1,1){
            //down-left
            if y_container[y+2][x-2] == 'A' && y_container[y+3][x-3] == 'S' {
                println!("Found word at {},{} going down-left",y,x);
                output+=1;
            }

        }

        
    }
    
    println!("done {}",output);
    Ok(())

}
