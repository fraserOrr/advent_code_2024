use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
    
    let mut output: i64 = 0;
    let mut content: String = String::new();

    for line in buf_reader.lines() {
        
        content.extend(line);
        
    }

    
    let mut good_sections: Vec<&str> = Vec::new();
    let mut bad_sections: Vec<&str> = Vec::new(); 
    
    
    
    //can we break the string into everying following do and everything following donts.

    bad_sections = content.split("don't()").collect();
    good_sections.push(bad_sections[0]);


    for i in 1..bad_sections.len(){
        println!("Bad sections: {:?}", bad_sections[i]);
        println!();
        let good_back_to_bad: Vec<&str> = bad_sections[i].split("do()").collect();

        for i in 1..good_back_to_bad.len(){
            good_sections.push(good_back_to_bad[i]);
        }
    }
    //
    
    
    println!("good sections {:?}", &good_sections);

    for good in good_sections{
        output += compute_good_section(&good);
    }

    

    println!("lines good : {}",output);
    Ok(())

}

fn compute_good_section(content: &str)-> i64{
    let mut output: i64 = 0;
    let re = Regex::new(r"(mul\([\d]{1,3},[\d]{1,3}\))").unwrap();
    for (_, [computation]) in re.captures_iter(&content).map(|c| c.extract()) {
        //println!("found computation: {:?}", computation);

        output += compute_out_put(computation)

    };


    return  output;
}

fn compute_out_put(computation: &str)->i64{

    // get the two numbers
    let mut computer_numbers: Vec<i64>= Vec::new();
    let mut outNu: i64 = 1;
    let re = Regex::new(r"([\d]{1,3})").unwrap();

    for (_, [numbers]) in re.captures_iter(&computation).map(|c| c.extract()) {
        //println!("found numbers: {:?}", numbers);
        computer_numbers.push(numbers.parse::<i64>().expect("not number matched"))
       

    };

    for number in computer_numbers{
        outNu = outNu * number
    }
    //println!("return number: {}", outNu);
    return outNu
}