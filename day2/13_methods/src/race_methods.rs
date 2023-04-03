#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Self {
        Self { name: String::from(name), laps: Vec::new() }
    }

    fn add_laps(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_laps(70);
    race.add_laps(68);
    race.print_laps();
    race.add_laps(71);
    race.print_laps();
    race.finish();
    // race.add_laps(42);
}