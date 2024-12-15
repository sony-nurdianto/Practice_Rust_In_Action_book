#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        Self { id: self.id }
    }
}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        match &self {
            Self::Ok => Self::Ok,
        }
    }
}

fn main() {
    let cube_sat = CubeSat { id: 64 };
    let cube_sat_a = cube_sat.clone();
    let cube_sat_b = cube_sat.clone();

    println!("{:?}", cube_sat_a);
    println!("{:?}", cube_sat_b);
    println!("{:?}", cube_sat);

    let status_msg = StatusMessage::Ok;
    let status_msg_a = status_msg.clone();
    let status_msg_b = status_msg.clone();

    println!("{:?}", status_msg);
    println!("{:?}", status_msg_a);
    println!("{:?}", status_msg_b);
}
