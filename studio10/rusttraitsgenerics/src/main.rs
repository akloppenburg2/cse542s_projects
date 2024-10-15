use std::fmt;

// Declare Directions Enum
#[derive(Debug)]
enum Directions {
    North,
    East,
    South,
    West,
}

// Implement display trait for Directions Enum
impl fmt::Display for Directions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Directions::North => write!(f, "North"),
            Directions::East => write!(f, "East"),
            Directions::South => write!(f, "South"),
            Directions::West => write!(f, "West"),
        }
    }
}

// Declare trait Mover
trait Mover {
    fn advance(&mut self);
}

// Declare Enum Rotation
enum Rotation {
    Left,
    Right,
}

// Declare trait Turner
trait Turner {
    fn turn(&mut self, r: Rotation);
}

// Declare struct Car
#[derive(Debug)]
struct Car {
    name: String,
    x_pos: isize,
    y_pos: isize,
    dir: Directions,
}

// Implement block for Car struct
impl Car {
    // fn creating new instance of Car
    pub fn new() -> Self {
        Self {
            name: "Lightning".to_string(),
            x_pos: 0,
            y_pos: 0,
            dir: Directions::North,
        }
    }
}

// Implement Mover trait for Car
impl Mover for Car {
    // Advance the car one position depending on its direction
    fn advance(&mut self) {
        match self.dir {
            Directions::North => self.y_pos += 1,
            Directions::East => self.x_pos += 1,
            Directions::South => self.y_pos -= 1,
            Directions::West => self.x_pos -= 1,
        }
    }
}

// Implement Turner trait for Car
impl Turner for Car {
    // Turn car based on current facing direction
    fn turn(&mut self, r: Rotation) {
        match r {
            Rotation::Left => {
                match self.dir {
                    Directions::North => self.dir = Directions::West,
                    Directions::East => self.dir = Directions::North,
                    Directions::West => self.dir = Directions::South,
                    Directions::South => self.dir = Directions::East,
                }
            }
            Rotation::Right => {
                match self.dir {
                    Directions::North => self.dir = Directions::East,
                    Directions::East => self.dir = Directions::South,
                    Directions::West => self.dir = Directions::North,
                    Directions::South => self.dir = Directions::West,
                }
            }
        }
    }
}

// Car performs a figure 8 while printing every step
fn figure_eight<T: Mover + Turner + std::fmt::Debug> (c: &mut T) {
    c.advance();
    println!("var: {:?}", c);

    c.turn(Rotation::Left);
    c.advance();
    println!("var: {:?}", c);

    c.turn(Rotation::Left);
    c.advance();
    println!("var: {:?}", c);

    c.turn(Rotation::Left);
    c.advance();
    println!("var: {:?}", c);

    c.advance();
    println!("var: {:?}", c);

    c.turn(Rotation::Right);
    c.advance();
    println!("var: {:?}", c);

    c.turn(Rotation::Right);
    c.advance();
    println!("var: {:?}", c);

    c.turn(Rotation::Right);
    c.advance();
    println!("var: {:?}", c);
}

fn main() {

    #[cfg(oldexercise)]
    {
        let mut car = Car::new();
        println!("Car name: {}, x_pos: {}, y_pos: {}, direction: {}", car.name, car.x_pos, car.y_pos, car.dir.to_string());

        car.advance();
        println!("Car name: {}, x_pos: {}, y_pos: {}, direction: {}", car.name, car.x_pos, car.y_pos, car.dir.to_string());

        car.turn(Rotation::Right);
        println!("Turn Right! Car name: {}, x_pos: {}, y_pos: {}, direction: {}", car.name, car.x_pos, car.y_pos, car.dir.to_string());

        car.turn(Rotation::Right);
        println!("Turn Right! Car name: {}, x_pos: {}, y_pos: {}, direction: {}", car.name, car.x_pos, car.y_pos, car.dir.to_string());

        car.turn(Rotation::Right);
        println!("Turn Right! Car name: {}, x_pos: {}, y_pos: {}, direction: {}", car.name, car.x_pos, car.y_pos, car.dir.to_string());

        car.turn(Rotation::Right);
        println!("Turn Right! Car name: {}, x_pos: {}, y_pos: {}, direction: {}", car.name, car.x_pos, car.y_pos, car.dir.to_string());

        car.turn(Rotation::Left);
        println!("Turn Left! Car name: {}, x_pos: {}, y_pos: {}, direction: {}", car.name, car.x_pos, car.y_pos, car.dir.to_string());

        car.turn(Rotation::Left);
        println!("Turn Left! Car name: {}, x_pos: {}, y_pos: {}, direction: {}", car.name, car.x_pos, car.y_pos, car.dir.to_string());

        car.turn(Rotation::Left);
        println!("Turn Left! Car name: {}, x_pos: {}, y_pos: {}, direction: {}", car.name, car.x_pos, car.y_pos, car.dir.to_string());

        car.turn(Rotation::Left);
        println!("Turn Left! Car name: {}, x_pos: {}, y_pos: {}, direction: {}", car.name, car.x_pos, car.y_pos, car.dir.to_string());
    }
    
    let mut c = Car::new();

    figure_eight(&mut c);
}
