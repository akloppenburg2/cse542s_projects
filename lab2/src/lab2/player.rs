pub struct Player {
    pub name: String,
    pub lines: Vec<String>, // Each line the character speaks
    pub on_stage: bool,     // Status of the character (on stage or off stage)
}

impl Player {
    // Constructor for a new player
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            lines: Vec::new(),
            on_stage: false,
        }
    }

    // Method to add a line to the player
    pub fn add_line(&mut self, line: &str) {
        self.lines.push(line.to_string());
    }

    // Method for the player to enter the stage
    pub fn enter(&mut self) {
        self.on_stage = true;
        println!("{} enters the stage.", self.name);
    }

    // Method for the player to exit the stage
    pub fn exit(&mut self) {
        self.on_stage = false;
        println!("{} exits the stage.", self.name);
    }

    // Method for the player to speak their lines
    pub fn speak(&self) {
        for line in &self.lines {
            println!("{}: {}", self.name, line);
        }
    }
}