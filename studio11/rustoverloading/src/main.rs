use std::fmt;
use std::ops::{AddAssign, SubAssign};

// Declare Directions Enum
#[derive(Debug)]
enum Directions {
    North,
    East,
    South,
    West,
}

// Implement Display trait for Directions
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

// Declare Mover trait
trait Mover {
    fn advance(&mut self);
}

// Declare Rotation enum
enum Rotation {
    Left,
    Right,
}

// Declare Turner trait
trait Turner {
    fn turn(&mut self, r: Rotation);
}

// Declare Car Struct 
#[derive(Debug)]
struct Car {
    name: String,
    pos: Position,
    dir: Directions,
}

// Implement block for Car struct
impl Car {
    // Creates new instance of Car
    pub fn new() -> Self {
        Self {
            name: "Lightning".to_string(),
            pos: Position::new(0, 0),
            dir: Directions::North,
        }
    }

    // moves Car back to original position and direction
    pub fn home(&mut self) {
        self.dir = Directions::North;

        loop {
            if self.pos.x == 0 && self.pos.y == 0 {
                break;
            }

            if self.pos.x > 0 {
                self.pos -= Position::new(1, 0);
            } else if self.pos.x < 0 {
                self.pos += Position::new(1, 0);
            }

            if self.pos.y > 0 {
                self.pos -= Position::new(0, 1);
            } else if self.pos.y < 0 {
                self.pos += Position::new(0, 1);
            }
        }
    }
}

// Implement Mover trait for Car
impl Mover for Car {
    // Advance the car one position depending on its direction
    fn advance(&mut self) {
        match self.dir {
            Directions::North => self.pos += Position::new(0, 1),
            Directions::East => self.pos += Position::new(1, 0),
            Directions::South => self.pos -= Position::new(0, 1),
            Directions::West => self.pos -= Position::new(1, 0),
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

// Declare Position Struct
#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
}

// Implement Block for Position
impl Position {
    // Creates a new instance of Position
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
        }
    }
}

// Overload Add for Position Struct
impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Overload Sub for Position Struct
impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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
    
    let mut c = Car::new();
    println!("c = {:?}", c);

    c.turn(Rotation::Left);
    c.advance();
    c.turn(Rotation::Right);
    c.advance();
    println!("c = {:?}", c);

    figure_eight(&mut c);

    c.home();
    println!("c = {:?}", c);
}
