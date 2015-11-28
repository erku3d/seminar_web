mod matrix;
use matrix::*;

use std::thread;
use std::sync::Arc;

extern crate num_cpus;
extern crate time;
use time::*;


fn vec1(x: &Vec<i32>, y: &Vec<i32>) ->  Vec<i32>{
    vector_add(&x,&y).unwrap()
}


fn vec2(x: &Vec<i32>, y: &Vec<i32>) ->  Vec<i32>{

    let mut max_num_threads = num_cpus::get() *2 ;
    let mut offset = ((x.len() as i32) / (max_num_threads as i32) + 1) as usize;

    if max_num_threads > x.len(){
        max_num_threads = x.len();
        offset = 1;
    }

    let mut handle = Vec::new();

    let x_arc =  Arc::new(x.clone());
    let y_arc =  Arc::new(y.clone());

    for i in 0..max_num_threads {

        let left = i * offset;
        let mut right = left + offset;

        if right > x.len() {
            right = x.len();
        }

        let x_arc = x_arc.clone();
        let y_arc = y_arc.clone();

        handle.push( thread::spawn(move|| {
            vector_add_as_slice(&x_arc[left..right],&y_arc[left..right])
        }));
    }


    let mut c: Vec<i32> = Vec::new();

    for h in handle{
        for e in h.join().unwrap().unwrap(){
            c.push(e);
        }
    }

    c

}

fn main() {


    //let x = vec![1,2,3,4,5,6,7,8,9,10];
    //let y = vec![1,2,3,4,5,6,7,8,9,10];


    let mut x: Vec<i32> = Vec::new();
    let mut y: Vec<i32> = Vec::new();

    for i in 0..100000{
        x.push(1);
        y.push(1);
    }

    let mut t1 = precise_time_ns();
    let w = vec1(&x,&y);
    let mut t2 = precise_time_ns();

    println!("iter:     {}",t2-t1);

    t1 = precise_time_ns();
    let v = vec2(&x,&y);
    t2 = precise_time_ns();

    println!("parallel: {}",t2-t1);

    println!("done");
    //println!("{:?}", w);
    //println!("{:?}", v);


}
