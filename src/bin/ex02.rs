use ready_set_boole::gray_code;

fn main() {
    for n in 0..=8 {
        println!("{}", gray_code(n));
    }
}
