extern crate time;
use time::*;


fn matrix_mul(a: &Vec<i32>,b: &Vec<i32>,l: usize,m: usize,n: usize) -> Vec<i32>{

    let r = l * n;  //Anzahl der Elemente der Matrix c

    //c hat l Zeilen und n Spalten

    let mut c: Vec<i32> = vec![0;r];

    for i in 0..r{
        for j in 0..m{
            c[i] = c[i] + (a[i / n * m + j] * b[(i % n) + j * n]);
        }
    }

    c
}

fn matrix_mul_2(a: &Vec<Vec<i32>>,b: &Vec<Vec<i32>>,l: usize,m: usize,n: usize) ->  Vec<Vec<i32>>{

    let mut c: Vec<Vec<i32>> = Vec::new();

    for _ in 0..l{
        c.push(vec![1;n]);
    }

    for i in 0..l{
        for k in 0..n{
            for j in 0..m{
                c[i][k] = c[i][k] + a[i][j] * b[j][k];
            }
        }
    };

    c
}



fn main() {

    let dim: usize = 500;

    let mat_lin: Vec<i32> =vec![7;dim*dim];

    let mut mat: Vec<Vec<i32>> = Vec::new();

    for _ in 0..dim{
        mat.push(vec![7;dim]);
    }

    let mut t1 = precise_time_ns();
    matrix_mul(&mat_lin,&mat_lin,dim,dim,dim);
    let mut t2 = precise_time_ns();

    let t_lin = t2-t1;

    t1 = precise_time_ns();
    matrix_mul_2(&mat,&mat,dim,dim,dim);
    t2 = precise_time_ns();

    let t_norm = t2-t1;
    println!("lin:        {}",t_lin);
    println!("normal:     {}",t_norm);

    let per: f32 = (t_lin as f32)/(t_norm as f32) * 100.0;

    println!("{}", per );

}
