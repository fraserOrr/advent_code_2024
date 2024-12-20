use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;




fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let mut output: HashMap<(isize,isize), &char> = HashMap::new();

    let mut y_container: Vec<Vec<char>> = Vec::new();
    
    let mut radio_keeper: HashMap<char,Vec<(usize,usize)>> = HashMap::new();
    for(y, line) in buf_reader.lines().enumerate() {
        let content = line.expect("oops file error");
        let mut x_container: Vec<char> = Vec::new();
        for (x,tmp_char) in content.chars().enumerate() {
            x_container.push(tmp_char.clone());

            if tmp_char != '.'{
                
                if radio_keeper.contains_key(&tmp_char)==false{
                    let mut tmp: Vec<(usize,usize)> =Vec::new();
                    tmp.push((y,x));
                    radio_keeper.insert(tmp_char,tmp);
                }else{
                    let mut tmp: Vec<(usize,usize)> =Vec::new();
                    tmp = radio_keeper.get(&tmp_char).unwrap().clone();
                    tmp.push((y,x));
                    radio_keeper.insert(tmp_char,tmp);

                }
                let mut radios: Vec<(usize,usize)> = Vec::new();
            }

        }

        y_container.push(x_container);
    }

    let y_upper_bound = y_container.len() as isize;
    let y_lower_bound = 0 as isize;
    let x_upper_bound = y_container[0].len() as isize;
    let x_lower_bound = 0;

    println!("For this map contrainsts are {} < y < {} and {} < x < {}",y_lower_bound,y_upper_bound,x_lower_bound,x_upper_bound);
    for signal_type in radio_keeper.iter(){

        println!("Antenna {} can be found at {:?}", signal_type.0, signal_type.1);

        //interference points 

        for i in 0..signal_type.1.len(){
            let location_1 = signal_type.1[i];

            for j in i+1..signal_type.1.len(){
                let location_2 = signal_type.1[j];
                println!("comparing {:?} to {:?}", location_1, location_2);
                let y_diff: isize = location_2.0  as isize  - location_1.0  as isize ;
                let x_diff: isize  = location_2.1  as isize  - location_1.1  as isize ;

                //get interfecer points

                
                let new_point_1: (isize,isize) = (location_1.0 as isize-y_diff,location_1.1 as isize-x_diff);
                let new_point_2: (isize,isize) = (location_2.0 as isize+y_diff ,location_2.1 as isize +x_diff);

                println!("test point 1 {:?}, test point 2 {:?}", new_point_1,new_point_2);                //are they in map
                if new_point_1.0 >= y_lower_bound && new_point_1.0 < y_upper_bound && new_point_1.1>=x_lower_bound && new_point_1.1<x_upper_bound{
                    //add to out list
                    if output.contains_key(&new_point_1)==false{
                        println!("found point at {:?}", new_point_1);
                        output.insert(new_point_1,signal_type.0);
                    }

                }

                if new_point_2.0 >= y_lower_bound && new_point_2.0 < y_upper_bound && new_point_2.1>=x_lower_bound && new_point_2.1<x_upper_bound{
                    //add to out list
                    if output.contains_key(&new_point_2)==false{
                        println!("found point at {:?}", new_point_2);
                        output.insert(new_point_2,signal_type.0);
                    }

                }

                

            }
        }

    }

    
    println!("done : {}", output.len());
    
    
    Ok(())

}
