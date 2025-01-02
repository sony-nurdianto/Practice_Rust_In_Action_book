use rand::{seq::SliceRandom, Rng};

#[derive(Debug)]
struct Dwarf {}

#[derive(Debug)]
struct Elf {}

#[derive(Debug)]
struct Human {}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket,
}

trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let probability_of_success = self.competency();
        let spell_is_successful = rand::thread_rng().gen_bool(probability_of_success);
        print!("{:?} mutter incoherently", self);
        if spell_is_successful {
            println!("The {:?} glows brightly", thing);
        } else {
            println!(
                "The {:?} glows brightly\
                then turn into worthless trinket.",
                thing
            );
            *thing = Thing::Trinket
        }
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.7
    }
}

fn main() {
    let mut thing = Thing::Sword;
    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};

    let party: Vec<&dyn Enchanter> = vec![&d, &e, &h];
    let spell_caster = party.choose(&mut rand::thread_rng()).unwrap();
    spell_caster.enchant(&mut thing);
}
