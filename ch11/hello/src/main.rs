macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}
macro_rules! vec {
    ($x:ty) => {
        {
            let temp_vec: Vec<$x> = Vec::new();
            temp_vec
        }
    };
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
fn main() {
    let count = five_times!(2 + 3);
    println!("{}", count);

    let vecs = vec![0];
    let y = vec![0, 2, 3];
    let z = vec![i32];
    println!("{:?}", vecs);
    println!("{:?}", y);
    println!("{:?}", z);
}

