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
            let mut edges: usize=0;
            if area==1{
                edges=4;
            }else{
                edges=RegionContainer::periminter_to_edges(&x.points);
            }

            println!("shape {} has area {} and edge {}",x.letter.to_string(),area,edges);
            return  area*edges;
        }).sum::<usize>();

        
    }
    fn periminter_to_edges(points: &Vec<(u64,u64)>)->usize{
        //let mut perimeter_hash: HashMap<(u64,u64), usize> = HashMap::new();
        let mut edge_hash: HashMap<(u64,u64), usize> = HashMap::new();
        let mut points = points.clone();
        points.iter_mut().for_each(|point|{ point.0+=4;point.1+=4});
        let mut x_high: u64 = 0;
        let mut y_high: u64 = 0;
        for point in points.iter(){
            //perimeter_hash.insert(point.clone(), 4);
            
            edge_hash.insert(point.clone(), 0);

            if point.0 > y_high {
                y_high = point.0
            }

            if point.1 > x_high {
                x_high = point.1
            }
        }
        let mut test_points:  Vec<(u64,u64)>= Vec::new();
        for y in 0..y_high+1{
            for x in 0..x_high+1{
                if  edge_hash.contains_key(&(y,x))==false{
                    test_points.push((y,x));
                }
                
            }
        }

        for test_point in test_points.iter(){
            for i in 0..points.len(){
                
                if RegionContainer::are_points_related(&points[i], &test_point)==true {
                    let tmp_ref =edge_hash.get_mut(&points[i]).unwrap();
                    *tmp_ref+=1;
                    
                }
                
            }
        }
        

        
        return edge_hash.into_values().sum();
    }
    
}

fn main() -> Result<(),Box<dyn std::error::Error>>{
   
    let file1 = File::open("test_input.txt")?;
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