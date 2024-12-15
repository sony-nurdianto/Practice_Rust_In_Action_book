#[derive(Debug, Clone, Copy)]
struct CubeSat {
    id: u64,
}

#[derive(Debug, Clone, Copy)]
enum StatusMessage {
    Ok,
}

#[allow(unused_variables)]
fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a);

    println!("a:{:?}", a_status.clone());
    println!("Hello, world!");

    let a_status = check_status(sat_a);
    println!("a:{:?}", a_status.clone());
}
