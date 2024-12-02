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

        if determine_if_good(&num_content,true) == true{
            count_of_good +=1;
            println!("Line parsed: {:?} is good", num_content)
        }else{
            println!("Line parsed: {:?} is bad", num_content)
        }


    }
    println!("lines good : {}", count_of_good);
    Ok(())

}

fn determine_if_good(report: &Vec<i64>, can_loop:bool)->bool{

    let mut direction: i64 = 0;
    let mut outcome: bool = true;
    let mut removed_once: bool = false;
    let mut i: usize= 1;
    while i < report.len(){

        
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
                
            }else if report[i-1]>report[i] && direction != -1{

                outcome=false;
                
            } 

        }
        let diff = (report[i-1]-report[i]).abs();

        if diff >= 1 && diff <= 3{
             
        }else{
            outcome=false;
            
        }
        
        
        if outcome==false && removed_once==false && can_loop==true {
            //make smaller vector to check

            removed_once=true;

            let mut smaller_vector_1: Vec<i64> = report[0..i-1].to_vec();
            let mut smaller_vector_2: Vec<i64> = report[i..report.len()].to_vec();
            smaller_vector_1.append(&mut smaller_vector_2);

            if determine_if_good(&smaller_vector_1, false) == true{
                outcome=true;
                println!("output is fixed by removing: {}", &report[i-1]);
                i+=1;

                //reset direction
                direction=0;

            }else{
                let mut smaller_vector_1: Vec<i64> = report[0..i].to_vec();
                let mut smaller_vector_2: Vec<i64> = report[i+1..report.len()].to_vec();
                smaller_vector_1.append(&mut smaller_vector_2);

                if determine_if_good(&smaller_vector_1, false) == true{
                    outcome=true;
                    println!("output is fixed by removing : {}", &report[i]);
                    i+=2;
                    if i > report.len(){
                        break;
                    }
                    //reset direction
                    direction=0;

                }
            }
            
            

            
        }

        if outcome == false{

            print!("line broke on : {} and : {}    ", report[i-1],report[i]);
            break;
        }

        i+=1;
    }

    return outcome;
}