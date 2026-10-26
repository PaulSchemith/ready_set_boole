use ready_set_boole::sat;

fn main() {
    println!("AB| -> {}", sat("AB|"));
    println!("AB& -> {}", sat("AB&"));
    println!("AA!& -> {}", sat("AA!&"));
    println!("AA^ -> {}", sat("AA^"));
}
