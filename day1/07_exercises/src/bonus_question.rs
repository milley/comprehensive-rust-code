use std::convert::AsRef;
use std::fmt::Debug;

fn pretty_print<T, Line, Matrix>(matrix: Matrix)
where
    T: Debug,
    Line: AsRef<[T]>,
    Matrix: AsRef<[Line]>,
{
    for row in matrix.as_ref() {
        println!("{:?}", row.as_ref());
    }
}

fn main() {
    // &[&[i32]]
    pretty_print(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
    // [[&str; 2]; 2]
    pretty_print([["a", "b"], ["c", "d"]]);
    // Vec<Vec<i32>>
    pretty_print(vec![vec![1, 2], vec![3, 4]]);
}
