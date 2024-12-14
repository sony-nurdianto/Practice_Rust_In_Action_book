#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

type Message = String;

fn parse_log(line: &str) -> (Event, Message) {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    if parts.len() == 1 {
        return (Event::Unknown, String::from(line));
    }

    let ev = parts[0];
    let res = String::from(parts[1]);

    match ev {
        "UPDATE" | "update" => (Event::Update, res),
        "DELETE" | "delete" => (Event::Delete, res),
        _ => (Event::Unknown, String::from(line)),
    }
}

fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}
