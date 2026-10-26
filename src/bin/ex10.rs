use ready_set_boole::map;

fn main() {
    println!("map(0, 0) = {}", map(0, 0));
    println!("map(1, 0) = {}", map(1, 0));
    println!("map(0, 1) = {}", map(0, 1));
    println!("map(65535, 65535) = {}", map(65535, 65535));
}
