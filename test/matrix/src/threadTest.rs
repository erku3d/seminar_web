use std::thread;
use std::sync::{Arc, Mutex};
extern crate num_cpus;


pub fn hello_thread(a: &Vec<i32>) -> Vec<i32>{

    let c = Arc::new(Mutex::new(vec![0;a.len()]));


    let mut handle = Vec::new();

    //let a = [3,2,1,1,0,2];

    let ma =  Arc::new(a.clone());

    let max_num_threads = num_cpus::get();

    println!("number of Threads: {}", max_num_threads);

    for i in 0..max_num_threads {
        let c = c.clone();
        let ma = ma.clone();

        handle.push( thread::spawn(move|| {
             //println!("Hello from a thread! {}",i);
             let mut index = i;
             let offset = max_num_threads;

             while index < ma.len() {
                 println!("thread {} working on Element {}",i,index);
                 let mut c = c.lock().unwrap();
                 c[index] += 1;
                index +=offset;
                //thread::sleep_ms(50);
             }

        }) );
    }

    for h in handle{
        h.join().unwrap();
    }

    return c.lock().unwrap().clone();

    //println!("{:?}\n{:?}", ca, a);

    //println!("{}", handle.join().unwrap());
}
