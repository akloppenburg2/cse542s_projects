#[derive(Debug)]
struct Point<T> {
    f1: T,
    f2: T
}

impl<T> Point<T> {
    pub fn new(f1: T, f2: T) -> Self {
        Self {
            f1,
            f2
        }
    }

    pub fn set(&mut self, val1: T, val2: T) {
        self.f1 = val1;
        self.f2 = val2;
    }
}

fn main() {
    
    let mut pt = Point::<f64>::new(3.0, 4.0);
    println!("pt is {:?}", pt);

    pt.set(33.0, 253.5);
    // pt.f1 = 33.0;
    // pt.f2 = 253.5;
    println!("pt is now {:?}", pt);

    let mut pt2 = Point::<usize>::new(8, 2);
    println!("pt2 is {:?}", pt2);

    pt2.set(5, 10);
    // pt2.f1 = 5;
    // pt2.f2 = 10;
    println!("pt2 is {:?}", pt2);
}
