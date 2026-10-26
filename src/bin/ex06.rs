use ready_set_boole::conjunctive_normal_form;

fn main() {
    for f in ["AB&!", "AB|!", "AB|C&", "AB|C|D|", "AB&C&D&", "AB&!C!|", "AB|!C!&", "ABCD&|&"] {
        println!("{f} -> {}", conjunctive_normal_form(f));
    }
}
