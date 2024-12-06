use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;



fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let mut y_container: Vec<Vec<char>> = Vec::new();
    let mut blocker: Vec<(usize,usize)> = Vec::new();
    let mut direction: &str = "up";
    let mut curr_location: (usize,usize) = (0,0);
    for(y, line) in buf_reader.lines().enumerate() {
        let content = line.expect("oops file error");
        let mut x_container: Vec<char> = Vec::new();
        for (x,tmp_char) in content.chars().enumerate() {
            x_container.push(tmp_char.clone());

            if tmp_char == '#'{
                //println!("found start point at : {},{}",y,x);
                blocker.push((y,x));
            }else if tmp_char =='^'{
                curr_location=(y,x);
            }

        }

        y_container.push(x_container);
    }


    //check map
    for x_item in y_container.iter(){
        println!("{:?}",x_item);
    }

    let y_upper_bound = y_container.len();
    let y_lower_bound = 0;
    let x_upper_bound = y_container[0].len();
    let x_lower_bound = 0;
    
    let mut game_ongoing: bool =true;
    let mut been_keeper: HashMap<(usize,usize), &str> = HashMap::new();

    while game_ongoing == true{
        if been_keeper.contains_key(&curr_location)==false{
            been_keeper.insert(curr_location,"#");
        }
        if direction == "up"{
            //am i going out of map
            if curr_location.0 > y_lower_bound{
                if blocker.contains(&(curr_location.0-1,curr_location.1)){
                    //found a blocker
                    //turn clockwise
                    direction="right";
                    
                }else{

                    //no blocker move 
                    curr_location = (&curr_location.0-1,curr_location.1);
                    
                }
            }else{
                println!("guard left room at {},{}",curr_location.0,curr_location.1);
                game_ongoing=false;
                break;
            }
            //check above me
            
        }else if direction=="right"{
            if curr_location.1 < x_upper_bound{
                if blocker.contains(&(curr_location.0,curr_location.1+1)){
                    //found a blocker
                    //turn clockwise
                    direction="down";
                    
                }else{

                    //no blocker move 
                    curr_location = (curr_location.0,curr_location.1+1);
                    
                }
            }else{
                println!("guard left room at {},{}",curr_location.0,curr_location.1);
                game_ongoing=false;
                break;
            }
        }else if direction=="down"{
            if curr_location.0 < y_upper_bound{
                if blocker.contains(&(curr_location.0+1,curr_location.1)){
                    //found a blocker
                    //turn clockwise
                    direction="left";
                    
                }else{

                    //no blocker move 
                    curr_location = (curr_location.0+1,curr_location.1);
                    
                }
            }else{
                println!("guard left room at {},{}",curr_location.0,curr_location.1);
                game_ongoing=false;
                break;
            }
        }else if direction=="left"{
            if curr_location.1 > x_lower_bound{
                if blocker.contains(&(curr_location.0,curr_location.1-1)){
                    //found a blocker
                    //turn clockwise
                    direction="up";
                    
                }else{

                    //no blocker move 
                    curr_location = (curr_location.0,curr_location.1-1);
                    
                }
            }else{
                println!("guard left room at {},{}",curr_location.0,curr_location.1);
                game_ongoing=false;
                break;
            }
        }

        
    }

    println!("number of squares visited: {}", been_keeper.len());
    
    Ok(())

}
