//! Beschreibung der Crate


pub mod matrix;
use matrix::*;

/// Matrix mit Zeilen- und Spaltenanzahl, sowie den Elementen (zeilenweise) d
#[derive(Debug)]
pub struct MyMatrix{
    /// Zeilen
    rows: i32,
    /// Spalten
    cols: i32,
    /// Elemente als Vektor
    elem: Vec<i32>,
}

impl MyMatrix{
    /// Returns a Matrix
    ///
    /// # Arguments
    ///
    /// * `r` - Zeilen der Matrix
    /// * `c` - Spalten der Matrix
    /// * `elem` - Elementen als Vektor (zeilenweise) der Matrix
    pub fn new(r:i32, c:i32, elem:Vec<i32>) -> MyMatrix{
        MyMatrix{
            rows: r,
            cols: c,
            elem: elem,
        }
    }

}

///Returns Value + 2
///
/// # Arguments
///
/// * `x` - Integer
///
/// # Example
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to Rustdoc, it will even test it for you!
/// let y = testing(3);
/// ```
pub fn testing(x:i32) -> i32{
    x+2
}


fn main() {
    let x = vec![1,2,3];
    let y = vec![1,2,3];

    let z = vector_add(&x,&y,3);

    let m = MyMatrix::new(1,3,x);

    let y = testing(3);

    println!("{:?}",m );
    println!("{:?}",z );
    println!("{}",y );

}
