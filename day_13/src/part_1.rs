use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use regex::Regex;

#[derive(Clone, Debug)]
struct Machine{
    btnn_a: (u64,u64),
    bttn_b: (u64,u64),
    goal: (u64,u64),
    

}
#[derive(Clone, Debug)]
struct GameTick{
    position: (u64,u64),
    counter: i32,
    cost: u64,
}
#[derive(Clone, Debug)]
struct MachineHolder{
    machine_list: Vec<Machine>
}

impl MachineHolder {

    fn work_out_computation(&mut self)->u64{
        let mut container: &mut Vec<Machine> = &mut self.machine_list;
        let mut tokens: u64= 0;
        container.iter_mut().for_each(|machine: &mut Machine| {
            println!("starting machine {:?}", machine.goal);
            tokens+=MachineHolder::machine_lowest_cost(machine);

        });
        return tokens;
    }
    
    fn machine_lowest_cost(machine: &mut Machine)->u64{
        let mut outputs: Vec<GameTick>=Vec::new();
        let firsttick: GameTick = GameTick { position: (0,0), counter: 0 , cost: 0};
        outputs.push(firsttick);

        for i in 1..100{
            println!("round: {}, contain len: {}", i, outputs.len());
            let mut tmp_container: Vec<GameTick>=Vec::new();
            for last_tick in outputs.iter(){
                if last_tick.position>machine.goal{
                    break;
                }else if last_tick.counter==i-1{
                    //try button A
                    let new_tick_1: GameTick=GameTick { position: (last_tick.position.0 + machine.btnn_a.0,last_tick.position.1 + machine.btnn_a.1), counter: last_tick.counter+1 , cost: last_tick.cost+3};
                    //try button B
                    let new_tick_2: GameTick=GameTick { position: (last_tick.position.0 + machine.bttn_b.0,last_tick.position.1 + machine.bttn_b.1), counter: last_tick.counter+1 , cost: last_tick.cost+1};
                    tmp_container.push(new_tick_1.clone());
                    tmp_container.push(new_tick_2.clone());
                }
            }

            for item in tmp_container.drain(..){
                outputs.push(item);
            }
            outputs.retain(|f|{
                if f.counter<i-1 && f.position!=machine.goal {
                    return false;
                }else{
                    return  true;
                }

            });

        }
        let mut possible_tokens: Vec<u64> = Vec::new();
        for gametick in outputs.iter(){
            if gametick.position == machine.goal{
                possible_tokens.push(gametick.cost);
            }
        }
        possible_tokens.sort();
        println!("possible tokens: {:?}", possible_tokens);
        if possible_tokens.len()==0{
            println!("machine: {:?} needs lowest token {}", machine.goal, 0);
            return 0;
        }else{
            println!("machine: {:?} needs lowest token {}", machine.goal, possible_tokens[0]);
            return possible_tokens[0];
        }
        
    }

}
fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("test_input.txt")?;
    let buf_reader = BufReader::new(file1);
    let mut line_count = 0;
    let mut bttnA: (u64,u64) = (0,0);
    let mut bttnB: (u64,u64)=(0,0);
    let mut goal: (u64,u64)=(0,0);
    let x_regex = Regex::new(r"X[+=](:?[0-9]*)").unwrap();
    let y_regex = Regex::new(r"Y[+=](:?[0-9]*)").unwrap();
    let mut machine_container: MachineHolder = MachineHolder { machine_list: vec![] };

    for line in buf_reader.lines() {
        let content = line.expect("oops file error");
        let mut x: u64 = 0;
        let mut y: u64 = 0;
        if line_count==0{
            
            for (_, [capture]) in x_regex.captures_iter(&content).map(|c| c.extract()) {
                x = capture.parse::<u64>().expect("failed parsing string to x");
            }
            for (_, [capture]) in y_regex.captures_iter(&content).map(|c| c.extract()) {
                y = capture.parse::<u64>().expect("failed parsing string to x");
            }

            bttnA = (x,y);
            line_count+=1;
        }else if line_count==1{
            
            for (_, [capture]) in x_regex.captures_iter(&content).map(|c| c.extract()) {
                x = capture.parse::<u64>().expect("failed parsing string to x");
            }
            for (_, [capture]) in y_regex.captures_iter(&content).map(|c| c.extract()) {
                y = capture.parse::<u64>().expect("failed parsing string to x");
            }

            bttnB = (x,y);
            line_count+=1;
        }else if line_count==2{
            
            for (_, [capture]) in x_regex.captures_iter(&content).map(|c| c.extract()) {
                x = capture.parse::<u64>().expect("failed parsing string to x");
            }
            for (_, [capture]) in y_regex.captures_iter(&content).map(|c| c.extract()) {
                y = capture.parse::<u64>().expect("failed parsing string to x");
            }

            goal = (x,y);
            line_count+=1;
        }else if line_count==3 {

            let tmp_machine: Machine = Machine { btnn_a:bttnA, bttn_b: bttnB, goal: goal };
            machine_container.machine_list.push(tmp_machine);

            line_count=0;
        }else {
            panic!("got onto bad line")
        }
    }

    
    let output: u64 = machine_container.work_out_computation();
    println!("{:?}", machine_container);


    println!("done : {}", output);
    
    
    Ok(())

}
