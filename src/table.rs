use crate::formula::Node;

/// Distinct variables of a formula, in alphabetical order. "BAB&" -> ['A', 'B'].
pub fn variables(formula: &str) -> Vec<char> {
    let mut vars: Vec<char> = formula.chars().filter(char::is_ascii_uppercase).collect();
    vars.sort_unstable();
    vars.dedup();
    vars
}

/// Turns a number into values for the variables: its bits, first variable = highest bit.
/// With vars = [A, B, C], combination 5 = 0b101 gives A = 1, B = 0, C = 1.
/// The result is indexed by letter (index 0 = A), as `Node::eval` expects.
pub fn assignment(vars: &[char], combination: u64) -> [bool; 26] {
    let mut values = [false; 26];
    for (i, var) in vars.iter().enumerate() {
        let bit = vars.len() - 1 - i;
        values[(*var as u8 - b'A') as usize] = (combination >> bit) & 1 == 1;
    }
    values
}

fn digit(b: bool) -> char {
    if b { '1' } else { '0' }
}

pub fn print_truth_table(formula: &str) {
    let tree = match Node::parse(formula) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };
    let vars = variables(formula);

    // Header: "| A | B | = |" then "|---|---|---|"
    let mut header = String::from("|");
    let mut separator = String::from("|");
    for v in &vars {
        header.push_str(&format!(" {v} |"));
        separator.push_str("---|");
    }
    println!("{header} = |");
    println!("{separator}---|");

    // One row per combination, from all 0 to all 1.
    for combination in 0..1u64 << vars.len() {
        let values = assignment(&vars, combination);
        let mut row = String::from("|");
        for v in &vars {
            row.push_str(&format!(" {} |", digit(values[(*v as u8 - b'A') as usize])));
        }
        println!("{row} {} |", digit(tree.eval(&values)));
    }
}
