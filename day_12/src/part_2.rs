use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::panic::resume_unwind;

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
                //create mutable copy
                let mut points = x.points.clone();
                edges=RegionContainer::periminter_to_edges(points);
            }

            println!("shape {} has area {} and edge {}",x.letter.to_string(),area,edges);
            return  area*edges;
        }).sum::<usize>();

        
    }
    fn return_total_edge(&mut self)->usize{

        return self.region_contrainer.iter().map(|x|{
            let area: usize =x.points.len();
            let mut edges: usize=0;
            if area==1{
                edges=4;
            }else{
                //create mutable copy
                let mut points = x.points.clone();
                edges=RegionContainer::periminter_to_edges(points);
            }

            //println!("shape {} has area {} and edge {}",x.letter.to_string(),area,edges);
            return  edges;
        }).sum::<usize>();

        
    }
    fn periminter_to_edges(points: Vec<(u64,u64)>)->usize{
        let mut points = points.clone();
        points.iter_mut().for_each(|f| {f.0+=5;f.1+=5;});
        let  (mut out_eddges,mut points_been)=RegionContainer::outedge(&mut points);
        //println!("out edged: {}", out_eddges);
        // how can we work out if the internal block space
        let mut y_lower: u64 =150;
        let mut y_higher: u64 =0;
        let mut x_lower: u64 =150;
        let mut x_higher: u64 =0;

        for point in points.iter(){
            if point.0 < y_lower{
                y_lower=point.0
            }
            if point.0 > y_higher{
                y_higher=point.0
            }
            if point.1 < x_lower{
                x_lower=point.1
            }
            if point.1 > x_higher{
               x_higher=point.1
            }
        }
        let mut region_holder2:RegionContainer =RegionContainer { region_contrainer: vec![] };



        for y in y_lower-15..y_higher+15{
            for x in x_lower-15..x_higher+15{
                if points.contains(&(y,x))==false{
                    let tmp: Region = Region { letter: '#', points: vec![(y as u64,x as u64)] };
                    region_holder2.region_contrainer.push(tmp);
                }
                
            }
        }
        region_holder2.sort_region();
        region_holder2.region_contrainer.pop();

        if region_holder2.region_contrainer.len()>0{
            //println!("found sub region {}", region_holder2.region_contrainer.len());
            
            //whats left
    
            //println!("{:?}", region_holder2);
            //work out result  
            let extra_bit = region_holder2.return_total_edge();
           // println!("adding on extra {}", extra_bit);
            out_eddges+=extra_bit
        }

        return out_eddges;


    }

    fn outedge(points: &mut Vec<(u64,u64)>)->(usize,Vec<(u64,u64)>){

        points.sort();
        let mut curr_point: (u64,u64) = points[0];
        let mut direction: &str = "U";
        
        let start_dir = direction.clone();
        let mut turns:usize =0;
        let mut points_been: Vec<(u64,u64)> = Vec::new();




        loop {
            curr_point = (curr_point.0,curr_point.1-1);
            if points.contains(&curr_point)==false{
                break;
            }
        }
        let starting_point = curr_point.clone();
        loop {
            
    
            if RegionContainer::check_impass(curr_point,RegionContainer::get_right(direction), &points, &mut points_been)==true{
                //do we have a wall to our right 
                //if we do can we go foward
                if RegionContainer::check_impass(curr_point, direction, &points, &mut points_been)==false{
                    curr_point=RegionContainer::walk_forward(curr_point, direction);
                }else{
                    //cant go forward and we have a wall to the right so turn left
                    
                    direction=RegionContainer::get_left(direction);
                    turns+=1;
                }
            }else{
                //if no turn right and move
                direction=RegionContainer::get_right(direction);
                curr_point=RegionContainer::walk_forward(curr_point, direction);
                turns+=1;

            }

            



            if curr_point==starting_point && start_dir==direction{
                break;
            }
        }
        



        return (turns,points_been)
    }
    fn check_impass(point:(u64,u64), direction: &str,points: &Vec<(u64,u64)>,points_been: &mut Vec<(u64,u64)>)->bool {
        if direction=="U"{

            if points.contains(&(point.0-1,point.1))==true{
                points_been.push((point.0-1,point.1));
                return true;
            }else {
                return false;
            }
        }else if direction =="R"{
            if points.contains(&(point.0,point.1+1))==true{
                points_been.push((point.0,point.1+1));
                return true;
            }else {
                return false;
            }
        }else if direction =="D"{
            if points.contains(&(point.0+1,point.1))==true{
                points_been.push((point.0+1,point.1));
                return true;
            }else {
                return false;
            }
        }else if direction =="L"{
            if points.contains(&(point.0,point.1-1))==true{
                points_been.push((point.0,point.1-1));
                return true;
            }else {
                return false;
            };
            

        }else {
            panic!("bad dir");
        };
    }

    fn walk_forward(point:(u64,u64), direction: &str)->(u64,u64){

        if direction=="U"{
            return (point.0-1,point.1);
        }else if direction =="R"{
            return (point.0,point.1+1);
        }else if direction =="D"{
            return (point.0+1,point.1);
        }else if direction =="L"{
            return (point.0,point.1-1);
        }else {
            panic!("bad dir");
        };
    }

    fn get_left( direction: &str)->&str{
        if direction=="U"{
            return "L";
        }else if direction =="R"{
            return "U";
        }else if direction =="D"{
            return "R";
        }else if direction =="L"{
            return "D";
        }else {
            panic!("bad dir");
        };
    }

    fn get_right( direction: &str)->&str{
        if direction=="U"{
            return "R";
        }else if direction =="R"{
            return "D";
        }else if direction =="D"{
            return "L";
        }else if direction =="L"{
            return "U";
        }else {
            panic!("bad dir");
        };
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