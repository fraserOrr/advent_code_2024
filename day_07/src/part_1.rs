use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Node{
    value: i64,
    depth: usize,
    total_value: i64,

}


fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let mut output: i64 = 0;
    for line in buf_reader.lines() {
        let content = line.expect("oops file error");
        let mut tmp: Vec<&str> = content.split(":").collect();
        let mut target: i64 = tmp[0].parse::<i64>().expect("couldnt convert string to num");
        println!("Target: {}", target);
        
        let mut inputs: Vec<i64> = tmp[1].split_whitespace().into_iter().map(

            |f| f.parse::<i64>().expect("failed to make num from str")
        
        ).collect();
        
        println!("Inputs: {:?}", inputs);
        //we got a target and inputs, now what
        let mut node_container: Vec<Node> = Vec::new();
        let mut  top_node: Node = Node { value: inputs[0].clone(),  depth: 0, total_value: inputs[0].clone(),  };
        node_container.push(top_node);
        for i in 1..inputs.len(){
            let curr_number = inputs[i];
            let mut tmp_collector: Vec<Node> = Vec::new();
            for node in node_container.iter(){
                if node.depth==i-1{
                    //add a plus node
                    let new_node_1 = Node{value: inputs[i],depth: i, total_value: node.total_value+inputs[i],};
                    //add a multi node
                    tmp_collector.push(new_node_1);
                    let new_node_2 = Node{value: inputs[i],depth: i, total_value: node.total_value*inputs[i],};
                    tmp_collector.push(new_node_2);
                }
            }

            for node in tmp_collector.iter(){
                node_container.push(node.clone());
            }
        }
        println!();
        println!("Nodes: {:?}", node_container);
        for node in node_container.iter(){
            if node.depth==inputs.len()-1 && node.total_value==target{
                println!();
                println!("found permuation");
                output+=node.total_value;
                break;
            }
        }
    }



    println!("done : {}", output);
    
    
    Ok(())

}
