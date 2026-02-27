enum Kind {
    Metadata = 0,
    TextNote = 1,
    RecommendRelay = 2,
}

fn main() {
    let kind = Kind::TextNote;
    println!("{}", kind as u32);
}