//! Crate für einfache Matrix berechnungen und so

/// Matrixmultiplikation
pub fn matrix_mul(a: &Vec<i32>,b: &Vec<i32>,l: usize,m: usize,n: usize) -> Vec<i32>{

    // l -> Zeilen von Matrix a
    // m -> Spalten von Matrix a bzw. Spalten von Matrix b
    // n -> Spalten von Matrix b

    let r = l * n;  //Anzahl der Elemente der Matrix c

    //c hat l Zeilen und n Spalten

    let mut c: Vec<i32> = vec![0;r];

    for i in 0..r{
        for j in 0..m{
            c[i] = c[i] + (a[i / n * m + j] * b[(i % n) + j * n]);
        }
    }

    return c;
}

///Matrixaddition
pub fn vector_add(a: &Vec<i32>,b: &Vec<i32>,s: usize) -> Vec<i32>{

    let mut c: Vec<i32> = Vec::new();

    for i in 0..s{
        c.push(a[i]+b[i]);
    }

    // println!("a: {:?}",a);
    // println!("b: {:?}",b);
    // println!("c: {:?}",c);
    return c;
}
