//! einfache Operationen auf Matrizen




/// Multiplikation zweier linearisierten Matrizen
///
/// # Argumente
///
/// * `a` - Matrix a Zeilenweise als Vektor
/// * `b` - Matrix b Zeilenweise als Vektor
/// * `l` - Zeilenanzahl von Matrix a
/// * `m` - Spaltenanzahl von Matrix a bzw. Spaltenanzahl von Matrix b
/// * `n` - Spaltenanzahl von Matrix b
///
/// # Examples
///
/// ```
///
/// let a = vec![3,2,1,1,0,2];
/// let b = vec![1,2,0,1,4,0];
///
/// let a_rows = 2; //Zeilen von a
/// let a_cols = 3; //Spalten von a bzw. Spalten  b
/// let b_cols = 2; //Spalten von b
///
/// let c = matrix_mul(&a, &b, a_rows, a_cols, b_cols).unwrap();
/// ```
pub fn matrix_mul(a: &Vec<i32>,b: &Vec<i32>,l: usize,m: usize,n: usize) ->  Option<Vec<i32>>{

    if (a.len() != l*m) || (b.len() != m*n){ //prüfe, ob beide Matrizen die Richtige Groeße haben
        return None;
    }

    let r = l * n;  //Anzahl der Elemente der Ergebnissmatrix c

    //c hat l Zeilen und n Spalten

    let mut c: Vec<i32> = vec![0;r]; //Vektor der laenge l mit 0 initialisiert

    for i in 0..r{
        for j in 0..m{
            c[i] = c[i] + (a[i / n * m + j] * b[(i % n) + j * n]);
        }
    }

    Some(c)
}

/// Addition zweier Vektoren
///
/// # Argumente
///
/// * `a` - Vektor a
/// * `b` - Vektor b
///
/// # Examples
///
/// ```
///
/// let a = vec![3,2,1];
/// let b = vec![1,2,3];
///
/// let c = vector_add(&a, &b).unwrap();
/// ```
pub fn vector_add(a: &Vec<i32>,b: &Vec<i32>) -> Option<Vec<i32>>{

    if a.len() != b.len(){
        return None
    }

    let mut c: Vec<i32> = Vec::new();

    for i in 0..a.len(){
        c.push(a[i]+b[i]);
    }

    Some(c)


}
