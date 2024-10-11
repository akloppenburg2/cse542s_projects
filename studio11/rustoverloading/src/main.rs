use std::fmt;
use std::ops::{AddAssign, SubAssign};

#[derive(Debug)]
enum Directions {
    North,
    East,
    South,
    West,
}

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

trait Mover {
    fn advance(&mut self);
}

enum Rotation {
    Left,
    Right,
}

trait Turner {
    fn turn(&mut self, r: Rotation);
}

#[derive(Debug)]
struct Car {
    name: String,
    pos: Position,
    dir: Directions,
}

impl Car {
    pub fn new() -> Self {
        Self {
            name: "Lightning".to_string(),
            pos: Position::new(0, 0),
            dir: Directions::North,
        }
    }

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

impl Mover for Car {
    fn advance(&mut self) {
        match self.dir {
            Directions::North => self.pos += Position::new(0, 1),
            Directions::East => self.pos += Position::new(1, 0),
            Directions::South => self.pos -= Position::new(0, 1),
            Directions::West => self.pos -= Position::new(1, 0),
        }
    }
}

impl Turner for Car {
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

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

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
