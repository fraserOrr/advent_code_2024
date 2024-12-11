use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;


#[derive(Debug, Clone, Copy,PartialEq )]
struct Trail {
    start: (usize,usize),
    curr_value: usize,
    curr_point: (usize,usize),
}

fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let mut output: usize=0;

    let mut y_container: Vec<Vec<usize>> = Vec::new();
    let mut trail_containers: Vec<Trail> = Vec::new();
    let mut start_points: Vec<(usize,usize)> = Vec::new();

    for(y, line) in buf_reader.lines().enumerate() {
        let content = line.expect("oops file error");
        let mut x_container: Vec<usize> = Vec::new();
        for (x,tmp_char) in content.chars().map(|c| c.to_digit(10).expect("failed to pass char to radix 10")).enumerate() {
            x_container.push(tmp_char as usize);
            if tmp_char == 0{
                start_points.push((y as usize,x as usize));
            }
            

        }

        y_container.push(x_container);
    }

    let y_upper_bound = y_container.len() as usize;
    let y_lower_bound = 0 as usize;
    let x_upper_bound = y_container[0].len() as usize;
    let x_lower_bound = 0 as usize;

    println!("For this map contrainsts are {} < y < {} and {} < x < {}",y_lower_bound,y_upper_bound,x_lower_bound,x_upper_bound);
    
    for x_item in y_container.iter(){
        println!("{:?}", x_item);
    }

    //lets find the starting trails
    for point in start_points.iter(){
        let y = point.0 as usize;
        let x = point.1 as usize;
        if y > y_lower_bound {
            if y_container[y-1][x]==1{
                let tmp_trail: Trail = Trail { start: (y,x), curr_value: 1, curr_point: (y-1,x) };
                trail_containers.push(tmp_trail);
            }
        }
        if  y+1 < y_upper_bound {
            if y_container[y+1][x]==1{
                let tmp_trail: Trail = Trail { start: (y,x), curr_value: 1, curr_point: (y+1,x) };
                trail_containers.push(tmp_trail);
            }
        }
        if  x > x_lower_bound {
            if y_container[y][x-1]==1{
                let tmp_trail: Trail = Trail { start: (y,x), curr_value: 1, curr_point: (y,x-1) };
                trail_containers.push(tmp_trail);
            }
        }
        if  x+1 < x_upper_bound {
            if y_container[y][x+1]==1{
                let tmp_trail: Trail = Trail { start: (y,x), curr_value: 1, curr_point: (y,x+1) };
                trail_containers.push(tmp_trail);
            }
        }


    }

    //got all the starting trails

    let mut loops_done: bool = false;
    let mut done_trail_containers: Vec<Trail> = Vec::new();
    while  loops_done==false {
        
        if trail_containers.len()==0{
            break;
        }
        let trail = trail_containers.pop().expect("no trails left");
        if trail.curr_value==9 {
            done_trail_containers.push(trail);
        }else{
            //println!("start: {:?}, curr {:?}, value {}", trail.start, trail.curr_point, trail.curr_value);
            let mut new_trails = check_trail(trail.clone(),&y_container);
            trail_containers.append(&mut new_trails);
        }
        
    }

    //count per start
    let mut scorer:HashMap<(usize,usize), i64> = HashMap::new();

 
    for trail in done_trail_containers.iter(){
        
        //println!("start: {:?}, curr {:?}, value {}", trail.start, trail.curr_point, trail.curr_value);
        if scorer.contains_key(&trail.start) ==true {
            let mut tmp = scorer.get_mut(&trail.start).expect("Expect value in hm");
            *tmp+=1;

        }else{

            scorer.insert(trail.start, 1);
        }
        
    }
    for (point, score) in scorer.iter(){
        println!("start: {:?} has score {}",point,score )
    }

    println!("done : {}", done_trail_containers.len());
    
    
    Ok(())

}

fn check_trail(trail: Trail, y_container: &Vec<Vec<usize>> ) -> Vec<Trail>{

    let y_upper_bound = y_container.len() as usize;
    let y_lower_bound = 0 as usize;
    let x_upper_bound = y_container[0].len() as usize;
    let x_lower_bound = 0 as usize;


    let mut return_vector: Vec<Trail> = Vec::new();
    let y = trail.curr_point.0;
    let x = trail.curr_point.1;
    let next_step = trail.curr_value+1;
    if y > y_lower_bound {
        if y_container[y-1][x]==next_step{
            let tmp_trail: Trail = Trail { start: trail.start, curr_value: next_step, curr_point: (y-1,x) };
            if return_vector.contains(&tmp_trail)==false{
                return_vector.push(tmp_trail);
            }
            
        }
    }
    if  y+1 < y_upper_bound {
        if y_container[y+1][x]==next_step{
            let tmp_trail: Trail = Trail { start: trail.start, curr_value: next_step, curr_point: (y+1,x) };
            if return_vector.contains(&tmp_trail)==false{
                return_vector.push(tmp_trail);
            }
        }
    }
    if  x > x_lower_bound {
        if y_container[y][x-1]==next_step{
            let tmp_trail: Trail = Trail { start: trail.start, curr_value: next_step, curr_point: (y,x-1) };
            if return_vector.contains(&tmp_trail)==false{
                return_vector.push(tmp_trail);
            }
        }
    }
    if  x+1 < x_upper_bound {
        if y_container[y][x+1]==next_step{
            let tmp_trail: Trail = Trail { start: trail.start, curr_value: next_step, curr_point: (y,x+1) };
            if return_vector.contains(&tmp_trail)==false{
                return_vector.push(tmp_trail);
            }
        }
    }

    return return_vector;
}