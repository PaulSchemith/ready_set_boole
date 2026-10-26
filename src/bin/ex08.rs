use ready_set_boole::powerset;

fn main() {
    println!("{:?}", powerset(vec![]));
    println!("{:?}", powerset(vec![1, 2, 3]));
}
