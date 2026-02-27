#[derive(Debug)]
struct Event {
    kind: u32,
    tags: Vec<Vec<String>>,
    content: String,
}

fn build_deletion_event(target_id: &str) -> Event {
    Event {
        kind: 5,
        tags: vec![vec!["e".to_string(), target_id.to_string()]],
        content: "Event deleted".to_string(),
    }
}

fn main() {
    let deletion = build_deletion_event("abc123eventid");
    println!("{:?}", deletion);
}