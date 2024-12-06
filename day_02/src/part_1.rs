use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
    
    let mut count_of_good: i64 = 0;
    for line in buf_reader.lines() {
        let content = line.expect("oops file error");

        let line_content: Vec<&str> = content.split(" ").collect();        
        let num_content: Vec<i64> = line_content.iter().map(|f| f.parse::<i64>().unwrap()).collect();

        //println!("Line parsed: {:?}", num_content)

        if determine_if_good(&num_content) == true{
            count_of_good +=1;
            println!("Line parsed: {:?} is good", num_content)
        }else{
            println!("Line parsed: {:?} is bad", num_content)
        }


    }
    println!("lines good : {}", count_of_good);
    Ok(())

}

fn determine_if_good(report: &Vec<i64>)->bool{

    let mut direction: i64 = 0;
    let mut outcome: bool = false;
    for i in 1..report.len(){

        if direction == 0{
            //determine direction
            if report[i-1]<report[i]{
                //going up
                direction=1;
            }else{

                direction=-1;
            }
            
        }else{
            if report[i-1]<report[i] && direction != 1{
                outcome=false;
                break;
            }else if report[i-1]>report[i] && direction != -1{

                outcome=false;
                break;
            } 

        }
        let diff = (report[i-1]-report[i]).abs();

        if diff >= 1 && diff <= 3{
            outcome=true;   
        }else{
            outcome=false;
            break;
        }
        

    }

    return outcome;
}