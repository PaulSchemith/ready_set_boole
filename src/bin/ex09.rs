use ready_set_boole::eval_set;

fn main() {
    println!("{:?}", eval_set("AB&", &[vec![0, 1, 2], vec![0, 3, 4]]));
    println!("{:?}", eval_set("AB|", &[vec![0, 1, 2], vec![3, 4, 5]]));
    println!("{:?}", eval_set("A!", &[vec![0, 1, 2]]));
}
