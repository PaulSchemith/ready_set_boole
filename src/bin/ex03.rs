use ready_set_boole::eval_formula;

fn main() {
    println!("10& = {}", eval_formula("10&"));
    println!("10| = {}", eval_formula("10|"));
    println!("11> = {}", eval_formula("11>"));
    println!("10= = {}", eval_formula("10="));
    println!("1011||= = {}", eval_formula("1011||="));
}
