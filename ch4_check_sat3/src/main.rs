use std::fmt::Display;

struct CubeSat {
    id: u64,
    mailbox: MailBox,
}

impl Display for CubeSat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "CubeSat {{id: {}, mailbox: Mailbox {}",
            self.id, self.mailbox
        )
    }
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.message.pop()
    }
}

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

struct MailBox {
    message: Vec<String>,
}

impl Display for MailBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // low level approach for learning proccess
        let mut message = String::new();
        for (i, item) in self.message.iter().enumerate() {
            if i != self.message.len() - 1 {
                message.push('"');
                message.push_str(item);
                message.push('"');
                message.push(' ');
            } else {
                message.push('"');
                message.push_str(item);
                message.push('"');
            }
        }
        write!(f, "MailBox {{ message: [{}] }}", message)

        // high level approach more idiomatic and for speed up development
        // write!(f, "MailBox {{ message: [{}] }}", self.message.join(','))
    }
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(to: &mut CubeSat, msg: Message) {
        to.mailbox.message.push(msg);
    }
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{}: {}", sat_id, StatusMessage::Ok);
    sat_id
}

#[allow(unused_variables)]
fn main() {
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: MailBox { message: vec![] },
    };

    println!("t0: {}", sat_a);

    GroundStation::send(&mut sat_a, Message::from("hello_there"));

    println!("t1: {}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {}", sat_a);

    println!("{:?}", msg);
}
