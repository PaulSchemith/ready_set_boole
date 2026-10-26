use ready_set_boole::adder;

fn main() {
    println!("0 + 0 = {}", adder(0, 0));
    println!("1 + 2 = {}", adder(1, 2));
    println!("123 + 877 = {}", adder(123, 877));

    // Cas limite : le résultat dépasse 32 bits, donc il retombe à 0.
    println!("4294967295 + 1 = {}", adder(4294967295, 1));
}
