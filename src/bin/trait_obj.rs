trait Playable {
    fn play(&self);
    //pause 方法可以不impl
    fn pause(&self) {
        println!("pause");
    }
    fn get_duration(&self) -> f32;
}

struct Audio {
    name: String,
    duration: f32,
}

impl Playable for Audio {
    fn play(&self) {
        println!("listening audio:{}", self.name)
    }

    fn get_duration(&self) -> f32 {
        self.duration
    }
}

struct Video {
    name: String,
    duration: f32,
}

impl Playable for Video {
    fn play(&self) {
        println!("watching video:{}", self.name)
    }

    fn pause(&self) {
        println!("video {} paused!", self.name)
    }

    fn get_duration(&self) -> f32 {
        self.duration
    }
}

fn main() {
    let x: &dyn Playable = &Audio {
        name: "以父之名.mp3".to_string(),
        duration: 4.0,
    };
    x.play();
    x.pause();

    let y: &dyn Playable = &Video {
        name: "蝙蝠侠.mp4".to_string(),
        duration: 120_f32,
    };
    y.play();
    y.pause();
}
