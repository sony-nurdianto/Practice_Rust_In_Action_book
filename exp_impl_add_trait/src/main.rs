use std::ops::Add;

#[derive(Debug)]
struct Experiment {
    x: i32,
    y: i32,
}

impl Experiment {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Experiment {
    type Output = Experiment;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let a: Experiment = Experiment { x: 2, y: 2 };
    let b: Experiment = Experiment::new(3, 3);

    let result: Experiment = a + b;

    println!("{} + {}", result.x, result.y);
}
