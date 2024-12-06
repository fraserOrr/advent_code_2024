
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
    let mut output: i64 = 0;
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

    let blocker: Vec<(usize,usize)> = blocker;
    let start_location= curr_location.clone();
    let start_direction = direction.clone();

    
    //check map
    for x_item in y_container.iter(){
        println!("{:?}",x_item);
    }

    let y_upper_bound = y_container.len();
    let y_lower_bound = 0;
    let x_upper_bound = y_container[0].len();
    let x_lower_bound = 0;
    
    for blocker_y in 0..y_upper_bound{
        println!("done: {}", blocker_y);
        for blocker_x in 0..x_upper_bound{
            direction=start_direction;
            curr_location=start_location;
            //println!("placed tmp blocker at {},{}",blocker_y,blocker_x);
            let mut tmp_blocker: Vec<(usize,usize)> = blocker.clone();
            tmp_blocker.push((blocker_y,blocker_x));
            let mut game_ongoing: bool =true;
            let mut been_keeper: HashMap<(usize,usize), Vec<&str>> = HashMap::new();

            while game_ongoing == true{
                if been_keeper.contains_key(&curr_location)==false{
                    let mut tmp: Vec<&str> =Vec::new();
                    tmp.push(direction);
                    been_keeper.insert(curr_location,tmp);
                }else{
                    let mut tmp: Vec<&str> =Vec::new();
                    tmp = been_keeper.get(&curr_location).unwrap().clone();
                    tmp.push(direction);
                    been_keeper.remove(&curr_location);
                    been_keeper.insert(curr_location,tmp);

                }
                if direction == "up"{
                    //am i going out of map
                    if curr_location.0 > y_lower_bound{
                        if tmp_blocker.contains(&(curr_location.0-1,curr_location.1)){
                            //found a blocker
                            //turn clockwise
                            direction="right";
                            
                        }else{

                            //no blocker move 
                            curr_location = (&curr_location.0-1,curr_location.1);
                            
                        }
                    }else{
                        //println!("guard left room at {},{}",curr_location.0,curr_location.1);
                        game_ongoing=false;
                        break;
                    }
                    //check above me
                    
                }else if direction=="right"{
                    if curr_location.1 < x_upper_bound{
                        if tmp_blocker.contains(&(curr_location.0,curr_location.1+1)){
                            //found a blocker
                            //turn clockwise
                            direction="down";
                            
                        }else{

                            //no blocker move 
                            curr_location = (curr_location.0,curr_location.1+1);
                            
                        }
                    }else{
                        //println!("guard left room at {},{}",curr_location.0,curr_location.1);
                        game_ongoing=false;
                        break;
                    }
                }else if direction=="down"{
                    if curr_location.0 < y_upper_bound{
                        if tmp_blocker.contains(&(curr_location.0+1,curr_location.1)){
                            //found a blocker
                            //turn clockwise
                            direction="left";
                            
                        }else{

                            //no blocker move 
                            curr_location = (curr_location.0+1,curr_location.1);
                            
                        }
                    }else{
                        //println!("guard left room at {},{}",curr_location.0,curr_location.1);
                        game_ongoing=false;
                        break;
                    }
                }else if direction=="left"{
                    if curr_location.1 > x_lower_bound{
                        if tmp_blocker.contains(&(curr_location.0,curr_location.1-1)){
                            //found a blocker
                            //turn clockwise
                            direction="up";
                            
                        }else{

                            //no blocker move 
                            curr_location = (curr_location.0,curr_location.1-1);
                            
                        }
                    }else{
                        //println!("guard left room at {},{}",curr_location.0,curr_location.1);
                        game_ongoing=false;
                        break;
                    }
                }

                if been_keeper.contains_key(&curr_location)==true{
                    let mut tmp: Vec<&str> =Vec::new();
                    tmp = been_keeper.get(&curr_location).unwrap().clone();
                    if tmp.contains(&direction){
                        println!("youve looped with blocker at {},{}",blocker_y,blocker_x);
                        output+=1;
                        break;
                    }
                }

                
            }
        }
    }

    

    println!("number of loops found: {}", output);
    
    Ok(())

}

