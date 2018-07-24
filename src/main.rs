extern crate rand;
use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Location {
    name: &'static str,
    hunting: &'static [&'static str],
    hunting_modifiers: &'static [&'static str],
    foraging: &'static [&'static str],
    foraging_modifiers: &'static [&'static str],
    locations: &'static [&'static str],
    max_creatures: u32,
    max_forage: u32,
}

impl Location {
    fn get_creature(&self) -> String {
        let mut rng = thread_rng();
        let creature = rng.choose(self.hunting).unwrap();

        let mut modifiers = self.hunting_modifiers.to_vec();
        rng.shuffle(&mut modifiers);
        modifiers.truncate(rng.gen_range(0, 3));
        let mut prefix = modifiers.join(" ");
        if prefix == "" {
            prefix = "mundane".to_string();
        }

        let location = rng.choose(self.locations).unwrap();

        format!("{} {} {}", prefix, creature, location)
    }

    fn get_forage(&self) -> String {
        let mut rng = thread_rng();
        let food = rng.choose(self.foraging).unwrap();

        let mut modifiers = self.foraging_modifiers.to_vec();
        rng.shuffle(&mut modifiers);
        modifiers.truncate(rng.gen_range(0, 3));
        let mut prefix = modifiers.join(" ");
        if prefix == "" {
            prefix = "mundane".to_string();
        }

        let location = rng.choose(self.locations).unwrap();

        format!("{} {} {}", prefix, food, location)
    }

    fn generate_and_print(&self) {
        println!("{}", self.name);

        println!("\nCreatures:");
        for i in 0..self.max_creatures {
            let creature = self.get_creature();
            println!("{}: {}", i + 1, creature);
        }

        println!("\nForaging:");
        for i in 0..self.max_forage {
            let food = self.get_forage();
            println!("{}: {}", i + 1, food);
        }
    }
}

fn main() {
    let forest = Location {
        name: "Forest",
        hunting: &["rabbit", "chicken"],
        hunting_modifiers: &["leafy", "moist", "majestic"],
        foraging: &["mushroom", "berries", "eggs"],
        foraging_modifiers: &["bountiful", "poison", "luminous", "slimy"],
        locations: &["under a tree", "by a stream", "in a clearing"],
        max_creatures: 10,
        max_forage: 4,
    };
    let desert = Location {
        name: "Desert",
        hunting: &["lizard", "snake"],
        hunting_modifiers: &["mundane", "sandy", "flaming", "obsidian", "dessicated"],
        foraging: &["cactus", "sand"],
        foraging_modifiers: &["dry", "sandy", "tasty"],
        locations: &["on a sand dune", "by a cactus", "in an oasis"],
        max_creatures: 4,
        max_forage: 2,
    };

    let locations = [forest, desert];
    let mut rng = thread_rng();

    let chosen_location = rng.choose(&locations).unwrap();

    chosen_location.generate_and_print();
}
