


pub fn matrix_mul(a: &Vec<i32>,b: &Vec<i32>,l: usize,m: usize,n: usize) -> Vec<i32>{
    let r = l * n;

    let mut c: Vec<i32> = vec![0;r];

    for i in 0..r{
        for j in 0..m{
            c[i] = c[i] + (a[i / n * m + j] * b[(i % n) + j * n]);
        }
    }

    return c;
}
