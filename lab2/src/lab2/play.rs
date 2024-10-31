use super::script_gen::Scene;

pub struct Play {
    pub title: String,
    pub scenes: Vec<Scene>,
}

impl Play {
    pub fn new(title: &str) -> Play {
        Play {
            title: title.to_string(),
            scenes: Vec::new(),
        }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    pub fn display(&self) {
        println!("Title: {}", self.title);
        for scene in &self.scenes {
            scene.display();
        }
    }
}