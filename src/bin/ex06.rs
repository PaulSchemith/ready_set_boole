use ready_set_boole::conjunctive_normal_form;

fn main() {
    println!("AB&! -> {}", conjunctive_normal_form("AB&!"));
    println!("AB|C& -> {}", conjunctive_normal_form("AB|C&"));
    println!("AB&!C!| -> {}", conjunctive_normal_form("AB&!C!|"));
    println!("ABCD&|& -> {}", conjunctive_normal_form("ABCD&|&"));
}
