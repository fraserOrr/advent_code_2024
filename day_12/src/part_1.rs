use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone)]
struct Region{
    letter: char,
    points: Vec<(u64,u64)>,

}
#[derive(Debug, Clone)]
struct RegionContainer{
    region_contrainer: Vec<Region>,
}



impl RegionContainer {

    fn sort_region(&mut self){
        //if i do a swap ill make it false
        let mut done: bool = false;
        //println!("starting region sort");
        let mut i: u64 =0;

        while i < self.region_contrainer.len() as u64{

            let start_len = self.region_contrainer.len();
            let mut curr_region =  self.region_contrainer.pop().unwrap();
            //println!("comparing region {:?}", curr_region);
            done=true;
            
            self.region_contrainer.retain(|other_region|{
                let mut inter_comparison: bool = false;
                if curr_region.letter==other_region.letter{
                    for point in curr_region.points.iter(){
                        for tmp_point in other_region.points.iter(){
                            if RegionContainer::are_points_related(point,tmp_point)==true{
                                inter_comparison=true;
                                //println!("region {:?} and {:?} overlap", curr_region, other_region)
                            }
                        }
                    } 
                }
                if inter_comparison==true{
                    curr_region.points.append(&mut other_region.clone().points);
                    return false;
                }
                return true;
            });
            self.region_contrainer.insert(0,curr_region);

            if start_len > self.region_contrainer.len(){
                i=1;
            }else{
                i+=1;
            }

        }
    }

    fn are_points_related(point1: &(u64,u64), point2: &(u64,u64))-> bool{
        if (point1.0.abs_diff(point2.0)==1 && point1.1==point2.1) ^(point1.1.abs_diff(point2.1)==1&& point1.0==point2.0) {
           return  true;
        }
        false
    }
    fn return_total_answer(&mut self)->usize{

        return self.region_contrainer.iter().map(|x|{
            let area: usize =x.points.len();
            let mut periminter: usize=0;
            if area==1{
                periminter=4;
            }else{
                periminter=RegionContainer::periminter(&x.points);
            }

            println!("shape {} has area {} and perimeter {}",x.letter.to_string(),area,periminter);
            return  area*periminter;
        }).sum::<usize>();

        
    }

    fn periminter(points: &Vec<(u64,u64)>)->usize{
        let mut shape_hash: HashMap<(u64,u64), usize> = HashMap::new();
        for point in points.iter(){
            shape_hash.insert(point.clone(), 4);
        }
        for i in 0..points.len(){
            for j in i..points.len(){
                if RegionContainer::are_points_related(&points[i], &points[j])==true{
                    let mut tmp_ref =shape_hash.get_mut(&points[i]).unwrap();
                    *tmp_ref-=1;
                    let mut tmp_ref =shape_hash.get_mut(&points[j]).unwrap();
                    *tmp_ref-=1;
                }
            }
        }

        
        return shape_hash.into_values().sum();
    }
}

fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("input.txt")?;
    let buf_reader = BufReader::new(file1);
    let mut region_holder:RegionContainer =RegionContainer { region_contrainer: vec![] };

    for(y, line) in buf_reader.lines().enumerate() {
        let content = line.expect("oops file error");       
        for (x,tmp_char) in content.chars().enumerate() {
            let tmp: Region = Region { letter: tmp_char, points: vec![(y as u64,x as u64)] };
            region_holder.region_contrainer.push(tmp);
        }
        
    }
    println!("regions going in {}", region_holder.region_contrainer.len());
    region_holder.sort_region();
    println!("regions going out {}", region_holder.region_contrainer.len());
    //whats left

    println!("{:?}", region_holder);
    //work out result  
    println!("done : {}", region_holder.return_total_answer());
    Ok(())

}