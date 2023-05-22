use std::ops::Add;

fn double<T>(i: T) -> T
where
    T: Add<Output = T> + Clone + Copy,
{
    i + i
}

fn main() {
    println!("{}", double(3_i8));
    println!("{}", double(4_i16));
}
