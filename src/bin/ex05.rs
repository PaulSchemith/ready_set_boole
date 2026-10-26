use ready_set_boole::negation_normal_form;

fn main() {
    println!("AB&! -> {}", negation_normal_form("AB&!"));
    println!("AB|! -> {}", negation_normal_form("AB|!"));
    println!("AB> -> {}", negation_normal_form("AB>"));
    println!("AB= -> {}", negation_normal_form("AB="));
    println!("AB|C&! -> {}", negation_normal_form("AB|C&!"));
}
