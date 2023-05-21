trait Area {
    fn get_area(&self) -> f64;
}

struct Square(f64);

struct Rectangle(f64, f64);

impl Area for Square {
    fn get_area(&self) -> f64 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f64 {
        self.0 * self.1
    }
}

fn main() {
    let mut sharps: Vec<&dyn Area> = vec![];
    sharps.push(&Square(3.0));
    sharps.push(&Rectangle(1.5, 1.5));
    println!("{}", sharps[0].get_area());
    println!("{}", sharps[1].get_area())
}
