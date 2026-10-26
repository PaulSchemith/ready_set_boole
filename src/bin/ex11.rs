use ready_set_boole::{map, reverse_map};

fn main() {
    for (x, y) in [(0, 0), (1, 0), (0, 1), (12345, 54321), (65535, 65535)] {
        let n = map(x, y);
        println!("reverse_map({n}) = {:?} (expected ({x}, {y}))", reverse_map(n));
    }
}
