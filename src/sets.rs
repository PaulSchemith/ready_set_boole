use crate::formula::Node;
use crate::table::variables;

pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let n = set.len();
    assert!(n < 64, "set too large");
    (0..1u64 << n)
        .map(|mask| (0..n).filter(|&i| (mask >> i) & 1 == 1).map(|i| set[i]).collect())
        .collect()
}

/// Evaluates a formula whose variables are sets (A = first set, B = second...).
/// The universe is the union of all the sets. Returns an empty list on invalid input.
pub fn eval_set(formula: &str, sets: &[Vec<i32>]) -> Vec<i32> {
    let tree = match Node::parse(formula) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {e}");
            return Vec::new();
        }
    };
    let vars = variables(formula);
    if vars.len() != sets.len() || vars.iter().any(|&v| (v as u8 - b'A') as usize >= sets.len()) {
        eprintln!("Error: the number of sets does not match the variables of the formula");
        return Vec::new();
    }

    let mut universe: Vec<i32> = sets.iter().flatten().copied().collect();
    universe.sort_unstable();
    universe.dedup();

    // For each element of the universe, "A is true" means "the element is in set A".
    // Keep the element if the formula is true for it.
    let mut result = Vec::new();
    for element in universe {
        let mut values = [false; 26];
        for (i, set) in sets.iter().enumerate() {
            values[i] = set.contains(&element);
        }
        if tree.eval(&values) {
            result.push(element);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn powerset_basic() {
        assert_eq!(powerset(vec![]), vec![Vec::<i32>::new()]);
        let p = powerset(vec![1, 2, 3]);
        assert_eq!(p.len(), 8);
        assert!(p.contains(&vec![1, 3]));
    }

    #[test]
    fn set_evaluation() {
        assert_eq!(eval_set("AB&", &[vec![0, 1, 2], vec![0, 3, 4]]), vec![0]);
        assert_eq!(
            eval_set("AB|", &[vec![0, 1, 2], vec![3, 4, 5]]),
            vec![0, 1, 2, 3, 4, 5]
        );
        assert_eq!(eval_set("A!", &[vec![0, 1, 2]]), Vec::<i32>::new());
        assert_eq!(eval_set("A!B&", &[vec![0, 1], vec![1, 2]]), vec![2]);
    }
}
