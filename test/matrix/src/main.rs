mod matrix;
use matrix::*;

mod threadTest;
use threadTest::*;



fn main() {

    let a = vec![3,2,1,1,0,2];
    let b = vec![1,2,4,5,0,1,3,2,4,0,0,1];

    let l: usize = 2;  //Zeilen von A
    let m: usize = 3;  //Spalten von A, Zeilen von B
    let n: usize = 4;  //Spalten von B

    //let c = matrix_mul(&a,&b,l,m,n);

    //println!("{:?}",c);

    let t = vec![7;1000];

    let c = hello_thread(&t);

    println!("{:?}\n{:?}", c, b);


}
