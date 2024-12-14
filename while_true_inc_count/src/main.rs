use std::time::{Duration, Instant};

fn main() {
    let numbers1 = vec![1, 2, 3, 4];
    let numbers2 = vec![10, 20, 30, 40];

    for (x, y) in numbers1.iter().zip(numbers2.iter()) {
        println!("x: {}, y: {}", x, y);
    }
    // let mut count = 0;
    // let time_limit = Duration::new(1, 0);
    // let start = Instant::now();
    //
    // println!("{:?}", &start);
    // while (Instant::now() - start) < time_limit {
    //     count += 1;
    // }
    // println!("{:?}", Instant::now());
    // println!("{}", count);
}
