use ready_set_boole::{map, reverse_map};

fn main() {
    // On encode (x, y) en un nombre avec map, puis on décode avec reverse_map.
    let n = map(12345, 54321);
    println!("map(12345, 54321) = {n}");
    println!("reverse_map({n}) = {:?}", reverse_map(n));
}
