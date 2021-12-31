use rusty_audio::Audio;

pub struct GameAudio {
    audio: Audio
}

impl GameAudio {
    pub fn new() -> Self {
        let mut audio = Audio::new();
        audio.add("explode", "./assets/sounds/explode.wav");
        audio.add("lose", "./assets/sounds/lose.wav");
        audio.add("move", "./assets/sounds/move.wav");
        audio.add("pew", "./assets/sounds/pew.wav");
        audio.add("startup", "./assets/sounds/startup.wav");
        audio.add("win", "./assets/sounds/win.wav");
        Self { audio }
    }

    pub fn explode(&mut self) {
        self.audio.play("explode");
    }

    pub fn lose(&mut self) {
        self.audio.play("lose");
    }

    pub fn moving(&mut self) {
        self.audio.play("move");
    }

    pub fn pew(&mut self) {
        self.audio.play("pew");
    }

    pub fn startup(&mut self) {
        self.audio.play("startup");
    }

    pub fn win(&mut self) {
        self.audio.play("win");
    }

    pub fn wait(&mut self) {
        self.audio.wait();
    }
}
