use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

struct Game{
    wall_map: Vec<(u64,u64)>,
    box_list: Vec<(u64,u64)>,
    robot_pos: (u64,u64),
    width: u64,
    height: u64,
}

impl Game {
    

    fn print_game(&mut self){

        for y in 0..self.height+1{
            for x in 0..self.width+1{
                if self.robot_pos==(x,y){
                    print!("{}",'@');
                }else if self.box_list.contains(&(x,y)) ==true{
                    print!("{}",'O');
                }else if self.wall_map.contains(&(x,y)) ==true{
                    print!("{}",'#');
                }else{
                    print!(".")
                }
            }
            println!()
        }
        
    }

    fn calculate_move(&mut self, move){

    }
}

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let map_file = "test_input_map.txt";
    let instruction_file = "test_input.txt";
    let file1 = File::open(map_file)?;
    let buf_reader = BufReader::new(file1);
    let output: i64 =0;
    let mut wall_map: Vec<(u64,u64)> = Vec::new();
    let mut box_list: Vec<(u64,u64)> = Vec::new();
    let mut robot_pos: (u64,u64) = (0,0);
    let mut width: u64 =0;
    let mut height: u64=0;

    for (y,line) in buf_reader.lines().enumerate() {
        let content = line.expect("oops file error");

        for (x,tmp_char) in content.chars().enumerate() {

            if tmp_char== '@'{
                robot_pos=(x as u64,y as u64);
            }else if  tmp_char== '#'{
                wall_map.push((x as u64,y as u64));
            }else if  tmp_char== 'O'{
                box_list.push((x as u64,y as u64));
            }
           
            
            width=x as u64;
        }
        
        height=y as u64;
    }

    let mut game: Game = Game { wall_map: wall_map, box_list: box_list, robot_pos: robot_pos, width:width,height:height };
    
    game.print_game();

    let file1 = File::open(instruction_file)?;
    let buf_reader = BufReader::new(file1);
    for line in buf_reader.lines() {
        let content = line.expect("oops file error");

        for tmp_char in content.chars() {


        }
            
    }

    println!("done : {}", output);
    

    
    Ok(())

}
