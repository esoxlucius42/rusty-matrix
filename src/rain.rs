use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Raindrop {
    pub x: usize,
    pub y: usize,
    pub length: usize,
    pub speed: usize,
    pub chars: [char; 32],
    pub char_count: usize,
}

pub struct RainSimulation {
    raindrops: Vec<Raindrop>,
    width: usize,
    height: usize,
    frame_count: u32,
    rng: rand::rngs::ThreadRng,
}

const CHARSET: &str = "ﾊﾐﾋｰｳﾆｻﾓﾗﾔﾏﾗﾁﾔﾜﾂｦﾘﾅﾆﾁﾎﾓﾆﾊﾐﾊﾁﾈﾌﾆﾈﾊﾐﾊﾏﾁﾔﾆﾘｦﾊﾏﾓﾈﾓﾅﾔﾏﾛﾇﾎﾜﾘﾍﾑﾀﾘﾅﾑﾊﾐﾎﾀﾏﾂｻﾗﾊﾈﾌﾊﾓﾐﾈﾁﾋﾋﾄﾁﾎﾈﾐﾜﾀﾌﾐﾔﾏﾊﾄﾂﾊﾏﾁﾔﾃﾏﾊﾊﾆﾈﾊﾐﾎﾊﾏﾐﾋﾓﾋﾎﾌﾆﾔﾀｦﾐﾜﾇﾛﾛﾌﾍﾘﾓﾆﾘﾃﾌﾊﾀﾉﾎﾅﾑﾓﾓﾏﾗﾎﾏﾁﾊﾜﾃﾌﾓﾊﾊﾑﾈﾊﾂﾃﾌﾊﾁﾔﾀﾊﾂﾘﾏﾎﾊﾊﾌﾋﾉﾋﾀﾌﾜﾀﾀﾆﾈﾌﾔﾀﾘﾂﾔﾘﾌﾀﾆﾌﾄﾂﾋﾜﾉﾐﾈﾂﾂﾋﾄﾀﾏﾁﾜﾃﾌﾄﾂﾄﾀﾘﾋﾠﾏﾁﾀﾀﾏﾀﾇﾅﾄﾃﾀﾘﾆﾘﾄﾂﾊﾂﾅﾈﾂﾕﾜﾓﾘﾆﾊﾂﾜﾊﾃﾀﾍﾌﾜﾛﾕﾊ0123456789:・\"'.,-ﾞﾟ";

impl RainSimulation {
    pub fn new(width: usize, height: usize) -> Self {
        let mut sim = Self {
            raindrops: Vec::new(),
            width,
            height,
            frame_count: 0,
            rng: rand::thread_rng(),
        };
        sim.spawn_raindrops();
        sim
    }

    fn spawn_raindrops(&mut self) {
        // Create initial raindrops across the width
        for x in (0..self.width).step_by(20) {
            self.create_raindrop(x);
        }
    }

    fn create_raindrop(&mut self, x: usize) {
        let length = self.rng.gen_range(10..30);
        let speed = self.rng.gen_range(1..4);

        let mut chars = [' '; 32];
        let mut char_count = 0;
        for _ in 0..length.min(32) {
            let char_idx = self.rng.gen_range(0..CHARSET.len());
            chars[char_count] = CHARSET.chars().nth(char_idx).unwrap_or('a');
            char_count += 1;
        }

        self.raindrops.push(Raindrop {
            x,
            y: 0,
            length,
            speed,
            chars,
            char_count,
        });
    }

    pub fn update(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);

        for raindrop in &mut self.raindrops {
            if self.frame_count % (5 - raindrop.speed.min(4)) as u32 == 0 {
                raindrop.y += 1;
            }
        }

        // Remove raindrops that are off screen and create new ones
        let mut i = 0;
        while i < self.raindrops.len() {
            if self.raindrops[i].y > self.height + self.raindrops[i].length {
                self.raindrops.remove(i);
            } else {
                i += 1;
            }
        }

        // Spawn new raindrops occasionally
        if self.frame_count % 5 == 0 && self.raindrops.len() < (self.width / 15) {
            let x = self.rng.gen_range(0..self.width);
            self.create_raindrop(x);
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.raindrops.clear();
        self.spawn_raindrops();
    }

    pub fn raindrops(&self) -> &[Raindrop] {
        &self.raindrops
    }
}
