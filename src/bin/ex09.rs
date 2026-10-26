use ready_set_boole::eval_set;

fn main() {
    // A = {0, 1, 2}, B = {0, 3, 4}
    let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];
    println!("AB& = {:?}", eval_set("AB&", &sets));
    println!("AB| = {:?}", eval_set("AB|", &sets));
    println!("A!B& = {:?}", eval_set("A!B&", &sets));
}
