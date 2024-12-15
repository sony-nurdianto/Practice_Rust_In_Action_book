use std::fmt::Display;

#[allow(unused_variables)]
enum StatusMessage {
    Ok,
}

impl Display for StatusMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "Ok"),
        }
    }
}

#[allow(unused_variables)]
fn check_status_stat(sat_id: i32) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = 0;
    let sat_b = 1;
    let sat_c = 2;

    let a_status = check_status_stat(sat_a);
    let b_status = check_status_stat(sat_b);
    let c_status = check_status_stat(sat_b);
    println!("a:{}, b:{}, c:{}", a_status, b_status, c_status);

    //waiting status
    let a_status = check_status_stat(sat_a);
    let b_status = check_status_stat(sat_b);
    let c_status = check_status_stat(sat_c);

    println!("a:{}, b:{}, c:{}", a_status, b_status, c_status);
}
