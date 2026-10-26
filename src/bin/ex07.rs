use ready_set_boole::sat;

fn main() {
    for f in ["AB|", "AB&", "AA!&", "AA^"] {
        println!("{f} -> {}", sat(f));
    }
}
