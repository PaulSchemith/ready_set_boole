use ready_set_boole::eval_formula;

fn main() {
    for f in ["10&", "10|", "11>", "10=", "1011||=", "1!"] {
        println!("{f} -> {}", eval_formula(f));
    }
}
