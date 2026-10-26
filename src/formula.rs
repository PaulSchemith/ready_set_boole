use crate::table::{assignment, variables};

/// A propositional formula as a binary tree.
/// Example: "AB&C|" is Or(And(Var('A'), Var('B')), Var('C')).
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Const(bool),
    Var(char),
    Not(Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Xor(Box<Node>, Box<Node>),
    Imp(Box<Node>, Box<Node>), // A => B
    Eq(Box<Node>, Box<Node>),  // A <=> B
}

use Node::*;

// Short constructors, so the code below stays readable.
fn not(a: Node) -> Node {
    Not(Box::new(a))
}
fn and(a: Node, b: Node) -> Node {
    And(Box::new(a), Box::new(b))
}
fn or(a: Node, b: Node) -> Node {
    Or(Box::new(a), Box::new(b))
}

impl Node {
    // ---------------------------------------------------------------- parsing

    /// Reads a formula in reverse polish notation using a stack:
    /// a value is pushed, an operator pops its operands and pushes the result.
    pub fn parse(formula: &str) -> Result<Node, String> {
        let mut stack: Vec<Node> = Vec::new();
        for c in formula.chars() {
            match c {
                '0' => stack.push(Const(false)),
                '1' => stack.push(Const(true)),
                'A'..='Z' => stack.push(Var(c)),
                '!' => {
                    let a = stack.pop().ok_or("missing operand for '!'")?;
                    stack.push(not(a));
                }
                '&' | '|' | '^' | '>' | '=' => {
                    // The right operand is on top of the stack.
                    let right = stack.pop().ok_or(format!("missing operand for '{c}'"))?;
                    let left = stack.pop().ok_or(format!("missing operand for '{c}'"))?;
                    let (l, r) = (Box::new(left), Box::new(right));
                    stack.push(match c {
                        '&' => And(l, r),
                        '|' => Or(l, r),
                        '^' => Xor(l, r),
                        '>' => Imp(l, r),
                        _ => Eq(l, r),
                    });
                }
                _ => return Err(format!("invalid character '{c}'")),
            }
        }
        let root = stack.pop().ok_or("empty formula")?;
        if !stack.is_empty() {
            return Err("too many operands".to_string());
        }
        Ok(root)
    }

    // ------------------------------------------------------------- evaluation

    /// `values[0]` is the value of A, `values[1]` of B, ... `values[25]` of Z.
    pub fn eval(&self, values: &[bool; 26]) -> bool {
        match self {
            Const(b) => *b,
            Var(c) => values[(*c as u8 - b'A') as usize],
            Not(a) => !a.eval(values),
            And(a, b) => a.eval(values) & b.eval(values),
            Or(a, b) => a.eval(values) | b.eval(values),
            Xor(a, b) => a.eval(values) ^ b.eval(values),
            Imp(a, b) => !a.eval(values) | b.eval(values),
            Eq(a, b) => a.eval(values) == b.eval(values),
        }
    }

    // ------------------------------------------------------------ normal forms

    /// Step 1 of NNF: rewrite `>`, `=` and `^` using only `!`, `&` and `|`.
    fn remove_sugar(&self) -> Node {
        match self {
            Const(_) | Var(_) => self.clone(),
            Not(a) => not(a.remove_sugar()),
            And(a, b) => and(a.remove_sugar(), b.remove_sugar()),
            Or(a, b) => or(a.remove_sugar(), b.remove_sugar()),
            // A => B   is   !A | B
            Imp(a, b) => or(not(a.remove_sugar()), b.remove_sugar()),
            // A = B    is   (A & B) | (!A & !B)
            Eq(a, b) => {
                let (a, b) = (a.remove_sugar(), b.remove_sugar());
                or(and(a.clone(), b.clone()), and(not(a), not(b)))
            }
            // A ^ B    is   (A & !B) | (!A & B)
            Xor(a, b) => {
                let (a, b) = (a.remove_sugar(), b.remove_sugar());
                or(and(a.clone(), not(b.clone())), and(not(a), b))
            }
        }
    }

    /// Step 2 of NNF (input must only contain `!`, `&`, `|`): push every `!` down to the variables.
    /// `negated` is true when an odd number of `!` sits above this node.
    fn push_negations(&self, negated: bool) -> Node {
        match self {
            Const(b) => Const(*b != negated),
            Var(_) if negated => not(self.clone()),
            Var(_) => self.clone(),
            // Double negation: !!A = A
            Not(a) => a.push_negations(!negated),
            // De Morgan: !(A & B) = !A | !B   and   !(A | B) = !A & !B
            And(a, b) if negated => or(a.push_negations(true), b.push_negations(true)),
            And(a, b) => and(a.push_negations(false), b.push_negations(false)),
            Or(a, b) if negated => and(a.push_negations(true), b.push_negations(true)),
            Or(a, b) => or(a.push_negations(false), b.push_negations(false)),
            _ => unreachable!("remove_sugar must be called first"),
        }
    }

    fn to_nnf(&self) -> Node {
        self.remove_sugar().push_negations(false)
    }

    /// CNF of a tree already in NNF: move every `|` below the `&`.
    fn to_cnf(&self) -> Node {
        match self {
            And(a, b) => and(a.to_cnf(), b.to_cnf()),
            Or(a, b) => distribute(a.to_cnf(), b.to_cnf()),
            _ => self.clone(), // variable or negated variable
        }
    }

    // ----------------------------------------------------------------- output

    /// Writes the tree in reverse polish notation.
    pub fn to_rpn(&self) -> String {
        match self {
            Const(true) => "1".to_string(),
            Const(false) => "0".to_string(),
            Var(c) => c.to_string(),
            Not(a) => format!("{}!", a.to_rpn()),
            And(..) => chain_rpn(self, '&'),
            Or(..) => chain_rpn(self, '|'),
            Xor(a, b) => format!("{}{}^", a.to_rpn(), b.to_rpn()),
            Imp(a, b) => format!("{}{}>", a.to_rpn(), b.to_rpn()),
            Eq(a, b) => format!("{}{}=", a.to_rpn(), b.to_rpn()),
        }
    }
}

/// (x | y) in CNF, where x and y are already in CNF.
/// Uses  (p & q) | y = (p | y) & (q | y)  and  x | (p & q) = (x | p) & (x | q).
fn distribute(x: Node, y: Node) -> Node {
    if let And(p, q) = x {
        and(distribute(*p, y.clone()), distribute(*q, y))
    } else if let And(p, q) = y {
        and(distribute(x.clone(), *p), distribute(x, *q))
    } else {
        or(x, y)
    }
}

/// Collects the operands of a chain like (A & (B & C)) & D  ->  [A, B, C, D].
fn operands<'a>(node: &'a Node, op: char, list: &mut Vec<&'a Node>) {
    match node {
        And(a, b) if op == '&' => {
            operands(a, op, list);
            operands(b, op, list);
        }
        Or(a, b) if op == '|' => {
            operands(a, op, list);
            operands(b, op, list);
        }
        _ => list.push(node),
    }
}

/// RPN of a chain of `&` (or `|`): all operands first, then the operators.
/// Example: ((A & B) & C) -> "ABC&&".
fn chain_rpn(node: &Node, op: char) -> String {
    let mut list = Vec::new();
    operands(node, op, &mut list);
    let mut out = String::new();
    for operand in &list {
        out.push_str(&operand.to_rpn());
    }
    for _ in 1..list.len() {
        out.push(op);
    }
    out
}

// ------------------------------------------------------------ public functions

/// Evaluates a formula made of `0`/`1` only. Prints an error and returns false if invalid.
pub fn eval_formula(formula: &str) -> bool {
    match Node::parse(formula) {
        Ok(_) if formula.chars().any(|c| c.is_ascii_uppercase()) => {
            eprintln!("Error: variables are not allowed here");
            false
        }
        Ok(tree) => tree.eval(&[false; 26]),
        Err(e) => {
            eprintln!("Error: {e}");
            false
        }
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    match Node::parse(formula) {
        Ok(tree) => tree.to_nnf().to_rpn(),
        Err(e) => {
            eprintln!("Error: {e}");
            String::new()
        }
    }
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    match Node::parse(formula) {
        Ok(tree) => tree.to_nnf().to_cnf().to_rpn(),
        Err(e) => {
            eprintln!("Error: {e}");
            String::new()
        }
    }
}

/// True if at least one combination of values makes the formula true.
pub fn sat(formula: &str) -> bool {
    let tree = match Node::parse(formula) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {e}");
            return false;
        }
    };
    let vars = variables(formula);
    // Try every combination: n variables -> 2^n combinations.
    (0..1u64 << vars.len()).any(|combination| tree.eval(&assignment(&vars, combination)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluation() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("11>"));
        assert!(!eval_formula("10="));
        assert!(eval_formula("1011||="));
        assert!(!eval_formula("11^"));
    }

    #[test]
    fn nnf_examples() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }

    #[test]
    fn cnf_examples() {
        assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
        assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
        assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
        assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
        assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
        assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
        assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
        assert_eq!(conjunctive_normal_form("ABCD&|&"), "ABC|BD|&&");
    }

    /// NNF and CNF must give the same result as the original on every combination.
    #[test]
    fn equivalence() {
        let formulas = ["AB=", "AB^C>", "AB&!C=", "ABC^=D|!", "AB>CD=^"];
        for f in formulas {
            let original = Node::parse(f).unwrap();
            let nnf = Node::parse(&negation_normal_form(f)).unwrap();
            let cnf = Node::parse(&conjunctive_normal_form(f)).unwrap();
            let vars = variables(f);
            for combination in 0..1u64 << vars.len() {
                let values = assignment(&vars, combination);
                assert_eq!(original.eval(&values), nnf.eval(&values), "nnf {f}");
                assert_eq!(original.eval(&values), cnf.eval(&values), "cnf {f}");
            }
        }
    }

    #[test]
    fn sat_examples() {
        assert!(sat("AB|"));
        assert!(sat("AB&"));
        assert!(!sat("AA!&"));
        assert!(!sat("AA^"));
    }

    #[test]
    fn invalid() {
        assert!(Node::parse("").is_err());
        assert!(Node::parse("A&").is_err());
        assert!(Node::parse("AB").is_err());
        assert!(Node::parse("a").is_err());
    }
}
