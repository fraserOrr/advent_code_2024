use core::num;
use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;



fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("rules.txt")?;
    let buf_reader1 = BufReader::new(file1);
    let mut rule_keeper: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut output: i64 = 0;
    for line in buf_reader1.lines() {
        let content = line.expect("oops file error");
        let mut tmp: Vec<&str> = content.split("|").collect();

        let k = tmp[0].parse::<i64>().expect("couldnt convert string to num");
        let v = tmp[1].parse::<i64>().expect("couldnt convert string to num");


        if rule_keeper.contains_key(&k) == true {
            let mut tmp_rule: Vec<i64> =Vec::new();
            tmp_rule = rule_keeper.get(&k).unwrap().clone();
            tmp_rule.push(v);
            rule_keeper.remove(&k);
            rule_keeper.insert(k,  tmp_rule);

        }else{
            let mut tmp_rule: Vec<i64> =Vec::new();
            tmp_rule.push(v);
            rule_keeper.insert(k,  tmp_rule);

        }
    }

    let file2 = File::open("input.txt")?;
    let buf_reader2 = BufReader::new(file2);
    let mut inputs: Vec<Vec<i64>>= Vec::new();
    for line in buf_reader2.lines() {
        let content = line.expect("oops file error");
        let mut tmp: Vec<&str> = content.split(",").collect();
        let mut input: Vec<i64> = Vec::new();
        
        for item in tmp.iter(){
            input.push(item.parse::<i64>().expect("failed to make num from str"));
        }

        inputs.push(input);
    }



    for (k,v) in rule_keeper.iter(){

        println!("{:?} comes before {:?}", k,v);
    }
    println!();
    for input in inputs.iter(){
        if check_input(&rule_keeper, &input) == true{
           // println!("Good input {:?}", input);
            //output+= input[((input.len()+1)/2)-1];
        }else{

            //fix bad ones
            let mut done = false;
            let mut tmp = input.clone();
            println!("bad value {:?}", input);
            while  done== false{
                
                tmp = fix_bad_ones(&rule_keeper, &tmp);

                if check_input(&rule_keeper, &tmp) == true{
                    println!("fixed {:?}", tmp);
                    done=true;
                }
            }

            output+= tmp[((tmp.len()+1)/2)-1];
        }
    }
    println!();




    println!("done {}", output);
    Ok(())

}

fn fix_bad_ones(rules: &HashMap<i64, Vec<i64>>, input: &Vec<i64>) ->  Vec<i64>{
    let mut output : Vec<i64> = Vec::new();
    output = input.clone();
    for i in (0..input.len()).rev(){
        //println!("checking {}",&input[i]);
        if rules.contains_key(&input[i])==true{

            let words_in_front: Vec<i64> = input[0..i].to_vec();
            let rule = rules.get(&input[i]).unwrap();
            for item in rule.iter(){
                if words_in_front.contains(item) == true{
                    let index = output.iter().position(|r| r == item).unwrap();
                    println!("Element {} in pos {}, needs to go in front of {} at {}",&input[i],i, item, index);
                    output.insert(index, input[i]);
                    
                    output.remove(i+1);
                    return output
                }
            }
            
        }
    }
    
    
    return output

}

fn check_input(rules: &HashMap<i64, Vec<i64>>, input: &Vec<i64>) -> bool{
    //println!("Input {:?}", input);
    let mut ret_bool: bool = true;
    //println!("Lenghth {}", input.len());
    for i in (0..input.len()).rev(){
        //println!("checking {}",&input[i]);
        if rules.contains_key(&input[i])==true{

            let words_in_front: Vec<i64> = input[0..i].to_vec();
            let rule = rules.get(&input[i]).unwrap();
            for item in rule.iter(){
                if words_in_front.contains(item) == true{
                    ret_bool = false;
                }
            }
            
        }
    }


    return ret_bool;
}