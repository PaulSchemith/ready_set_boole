use ready_set_boole::multiplier;

fn main() {
    for (a, b) in [(0, 5), (1, 1), (7, 6), (12345, 67), (65536, 65536)] {
        println!("{a} * {b} = {}", multiplier(a, b));
    }
}
