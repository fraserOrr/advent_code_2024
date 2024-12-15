use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Debug)]
struct Machine{
    btnn_a: (i64,i64),
    bttn_b: (i64,i64),
    goal: (i64,i64),
    

}
#[derive(Clone, Debug)]
struct MachineHolder{
    machine_list: Vec<Machine>
}

impl MachineHolder {
    fn work_out_computation2(&mut self)->i64{
        let container: &mut Vec<Machine> = &mut self.machine_list;
        let mut tokens: i64= 0;
        container.iter_mut().for_each(|machine: &mut Machine| {
            println!("starting machine {:?}", machine.goal);
            machine.goal = (machine.goal.0+10000000000000, machine.goal.1+10000000000000);
            let B: i64 = ((machine.btnn_a.0*machine.goal.1)-(machine.btnn_a.1*machine.goal.0)) / (    (machine.btnn_a.0 * machine.bttn_b.1) -(machine.btnn_a.1*machine.bttn_b.0)     );
            let B_rem: i64 = ((machine.btnn_a.0*machine.goal.1)-(machine.btnn_a.1*machine.goal.0)) % (    (machine.btnn_a.0 * machine.bttn_b.1) -(machine.btnn_a.1*machine.bttn_b.0)     );
            let A: i64 = ((machine.bttn_b.1* machine.goal.0) - (machine.bttn_b.0*machine.goal.1))/((machine.btnn_a.0*machine.bttn_b.1)-(machine.bttn_b.0*machine.btnn_a.1));
            let A_rem: i64 = ((machine.bttn_b.1* machine.goal.0) - (machine.bttn_b.0*machine.goal.1))%((machine.btnn_a.0*machine.bttn_b.1)-(machine.bttn_b.0*machine.btnn_a.1));
            if B_rem == 0 && A_rem==0{
                println!("min cost: {}", 3*A+B);
                tokens += 3*A+B;
            }else{
                tokens +=0
            }
        });
        return tokens;
    }


}
fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let mut line_count = 0;
    let mut bttnA: (i64,i64) = (0,0);
    let mut bttnB: (i64,i64)=(0,0);
    let mut goal: (i64,i64)=(0,0);
    let x_regex = Regex::new(r"X[+=](:?[0-9]*)").unwrap();
    let y_regex = Regex::new(r"Y[+=](:?[0-9]*)").unwrap();
    let mut machine_container: MachineHolder = MachineHolder { machine_list: vec![] };

    for line in buf_reader.lines() {
        let content = line.expect("oops file error");
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        if line_count==0{
            
            for (_, [capture]) in x_regex.captures_iter(&content).map(|c| c.extract()) {
                x = capture.parse::<i64>().expect("failed parsing string to x");
            }
            for (_, [capture]) in y_regex.captures_iter(&content).map(|c| c.extract()) {
                y = capture.parse::<i64>().expect("failed parsing string to x");
            }

            bttnA = (x,y);
            line_count+=1;
        }else if line_count==1{
            
            for (_, [capture]) in x_regex.captures_iter(&content).map(|c| c.extract()) {
                x = capture.parse::<i64>().expect("failed parsing string to x");
            }
            for (_, [capture]) in y_regex.captures_iter(&content).map(|c| c.extract()) {
                y = capture.parse::<i64>().expect("failed parsing string to x");
            }

            bttnB = (x,y);
            line_count+=1;
        }else if line_count==2{
            
            for (_, [capture]) in x_regex.captures_iter(&content).map(|c| c.extract()) {
                x = capture.parse::<i64>().expect("failed parsing string to x");
            }
            for (_, [capture]) in y_regex.captures_iter(&content).map(|c| c.extract()) {
                y = capture.parse::<i64>().expect("failed parsing string to x");
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

    
    let output: i64 = machine_container.work_out_computation2();
    //println!("{:?}", machine_container);


    println!("done : {}", output);
    
    
    Ok(())

}
