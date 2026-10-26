use ready_set_boole::map;

fn main() {
    for (x, y) in [(0, 0), (1, 0), (0, 1), (1, 1), (65535, 65535)] {
        println!("map({x}, {y}) = {}", map(x, y));
    }
}
