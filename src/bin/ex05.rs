use ready_set_boole::negation_normal_form;

fn main() {
    for f in ["AB&!", "AB|!", "AB>", "AB=", "AB|C&!"] {
        println!("{f} -> {}", negation_normal_form(f));
    }
}
