use std::fmt::Display;

// Note Using Display Trait is for Learning Purpose , efficient and almost common implementation is
// using Display "Atribute"

struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn recv(&self, mailbox: &mut MailBox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fecth_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

impl Display for CubeSat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CubeSat {{id: {}}}", self.id)
    }
}

struct MailBox {
    messages: Vec<Message>,
}

impl MailBox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recepient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recepient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

impl Display for MailBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //low level approach for learning purpose
        write!(f, "Mailbox{{messages: [")?;
        for (i, item) in self.messages.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "Message{{to:{},content:{}}}", item.to, item.content)?;
        }
        write!(f, "]}}")

        //high level approach for development or production
        // let messages: String = self
        //     .messages
        //     .iter()
        //     .map(|item| format!("Message{{to:{},content:{}}}", item.to, item.content))
        //     .collect::<Vec<_>>()
        //     .join(", ");
        //
        // write!(f, "Mailbox{{messages: [{}]}}", messages)
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message {{to: {},content:{}}}", self.to, self.content)
    }
}

struct GroundStation {}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(&self, mailbox: &mut MailBox, msg: Message) {
        mailbox.post(msg);
    }
}

#[allow(unused_variables)]
fn main() {
    let base = GroundStation {};
    let mut mail = MailBox { messages: vec![] };
    let sat_ids = fecth_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message {
            to: sat_id,
            content: String::from("hello"),
        };
        base.send(&mut mail, msg);
    }

    let sat_ids = fecth_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("{}: {:?}", sat, msg);
    }
}
