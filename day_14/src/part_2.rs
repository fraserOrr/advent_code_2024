use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::thread::sleep;

use regex::Regex;

#[derive(Debug)]
struct Robot{
    position:(i64,i64),
    velocity: (i64,i64),
}
#[derive(Debug)]
struct Map{
    robots: Vec<Robot>,
    width: i64,
    height: i64,
}

impl Map {
    
    fn game_tick(&mut self){

        self.robots.iter_mut().for_each(|robot|{

            //find new position

            robot.position= ( robot.position.0+robot.velocity.0 ,robot.position.1+robot.velocity.1);
            if robot.position.0 >= self.width {
                robot.position.0 = robot.position.0 - self.width
            }
            if robot.position.0 < 0 {
                robot.position.0 = robot.position.0 + self.width
            }
            if robot.position.1 >= self.height{
                robot.position.1 = robot.position.1 - self.height
            }
            if robot.position.1 < 0 {
                robot.position.1 = robot.position.1 + self.height
            }


        });
    }

    fn find_score(&mut self)->u64{

        let mut count_1: u64 =0;
        let mut count_2: u64 =0;
        let mut count_3: u64 =0;
        let mut count_4: u64 =0;

        println!("middle is {},{}",((self.width)/2),((self.height )/2));
        for cur_robot in self.robots.iter(){
            
            if cur_robot.position.0 < ((self.width)/2) && cur_robot.position.1 < ((self.height )/2){
                //quad 1 
                count_1+=1;
            }else if cur_robot.position.0 < ((self.width)/2) && cur_robot.position.1 > ((self.height)/2){
                //quad 2 
                count_2+=1;

            }else if cur_robot.position.0 > ((self.width)/2) && cur_robot.position.1 < ((self.height )/2){
                //quad 3 
                count_3+=1;

            }else if cur_robot.position.0 > ((self.width)/2) && cur_robot.position.1 > ((self.height )/2){
                //quad 4 
                count_4+=1;
            }
        }

        println!("quad 1: {}, quad 2: {}, quad 3: {}, quad 4: {}", count_1,count_2,count_3,count_4);
        return count_1*count_2*count_3*count_4;
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

    fn print_file(&mut self,count: u64){

        let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("output.txt")
        .unwrap();
        writeln!(file,"{}",count);
        for y in 0..self.height{
            for x in 0..self.width{
                let mut robo_count =0;
                for cur_robot in self.robots.iter(){
                    
                    if cur_robot.position == (x,y){
                        robo_count+=1;
                    }
                    

                }
                
                if robo_count==0{
                    write!(file,".").expect("file write error");
                }else{
                    write!(file,"{}", robo_count).expect("file write error");
                }
            }
            writeln!(file).expect("file write error");
        }
    }
}


fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let corrd_regex: Regex =  Regex::new(r"[-]?\d{1,5},[-]?\d{1,5}").unwrap();

    let width: i64 = 101;
    let height: i64 =103;


    let mut game: Map = Map { robots: vec![] ,width: width, height:height};
    
    for line in buf_reader.lines() {
        let content = line.expect("oops file error");
        let mut tmp_pos:(i64,i64)= (0,0);
        let mut tmp_vel:(i64,i64)= (0,0);
        let corrds: Vec<&str> = corrd_regex.find_iter(&content).map(|m| m.as_str()).collect();
        //println!("{:?}", corrds);
        let tmp: Vec<&str> = corrds[0].split(",").collect();
        tmp_pos = (tmp[0].parse::<i64>().expect("failed to parse input to number"),tmp[1].parse::<i64>().expect("failed to parse input to number"));
        let tmp2: Vec<&str> = corrds[1].split(",").collect();
        tmp_vel = (tmp2[0].parse::<i64>().expect("failed to parse input to number"),tmp2[1].parse::<i64>().expect("failed to parse input to number"));
        //println!("{:?} , {:?}",tmp_pos, tmp_vel);

        let robot: Robot = Robot { position: tmp_pos, velocity: tmp_vel };
        game.robots.push(robot);

    }

    
    
    println!("{:?}", game);
    game.print_board();
    println!("");

    for i in 0..16000{
        if i % 100 == 0{
            println!("{}",i);
        }
        
        game.game_tick();
        
        if i > 8000{
            game.print_file(i);
        }
        
        
        
        
    }
    println!("{:?}", game);
    game.print_board();
    println!("done : {}", game.find_score());
    

    
    Ok(())

}
