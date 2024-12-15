use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Robot{
    position:(u64,u64),
    velocity: (i64,i64),
}
#[derive(Debug)]
struct Map{
    robots: Vec<Robot>,
    width: u64,
    height: u64,
}

impl Map {
    
    fn game_tick(&mut self){

        self.robots.iter_mut().for_each(|robot|{

            //find new position

        });
    }

    fn print_board(&mut self){
        for y in 0..self.height{
            for x in 0..self.width{
                let mut robo_count =0;
                for cur_robot in self.robots.iter(){
                    
                    if cur_robot.position == (x,y){
                        robo_count+=1;
                    }
                    

                }

                if robo_count==0{
                    print!(".");
                }else{
                    print!("{}", robo_count);
                }
            }
            println!();
        }
    }
}


fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("test_input.txt")?;
    let buf_reader = BufReader::new(file1);
    let output: i64 = 0;
    let corrd_regex: Regex =  Regex::new(r"[-]?\d{1,5},[-]?\d{1,5}").unwrap();

    let width: u64 = 11;
    let height: u64 =7;


    let mut game: Map = Map { robots: vec![] ,width: width, height:height};
    
    for line in buf_reader.lines() {
        let content = line.expect("oops file error");
        let mut tmp_pos:(u64,u64)= (0,0);
        let mut tmp_vel:(i64,i64)= (0,0);
        let corrds: Vec<&str> = corrd_regex.find_iter(&content).map(|m| m.as_str()).collect();
        //println!("{:?}", corrds);
        let tmp: Vec<&str> = corrds[0].split(",").collect();
        tmp_pos = (tmp[0].parse::<u64>().expect("failed to parse input to number"),tmp[1].parse::<u64>().expect("failed to parse input to number"));
        let tmp2: Vec<&str> = corrds[1].split(",").collect();
        tmp_vel = (tmp2[0].parse::<i64>().expect("failed to parse input to number"),tmp2[1].parse::<i64>().expect("failed to parse input to number"));
        //println!("{:?} , {:?}",tmp_pos, tmp_vel);

        let robot: Robot = Robot { position: tmp_pos, velocity: tmp_vel };
        game.robots.push(robot);

    }

    
    
    println!("{:?}", game);
    game.print_board();

    println!("done : {}", output);
    
    
    Ok(())

}
