# Ready, Set, Boole! (projet 42)

Introduction à l'algèbre booléenne et à la théorie des ensembles. Sujet v3.0 (Tristan Duquesne, Luc Lenôtre). Dépôt vide au départ ; seul le travail dans le dépôt Git est évalué en soutenance.

## Langage et état de l'environnement

- Langage non imposé. Les prototypes du sujet sont en pseudo-code proche de Rust (`u32`, `println!`), donc **Rust recommandé** (génériques, closures, opérations bit à bit natives, surcharge d'opérateurs possible).
- **Rust choisi et implémenté** (rustup installé dans `~/.cargo`; faire `export PATH=$HOME/.cargo/bin:$PATH`).
- Structure : lib `src/` (`arith.rs` ex00-02, `formula.rs` AST/ex03/05/06/07, `table.rs` ex04, `sets.rs` ex08-09, `curve.rs` ex10-11) + un binaire de test par exercice dans `src/bin/exNN.rs`.
- Commandes : `cargo test` (tests unitaires avec les exemples du sujet), `cargo run --bin ex04`.
- ex10/11 : courbe de Morton, divisée par 2^32 (exact en f64, donc bijection exacte).
- Contraintes du langage : génériques, fonctions de première classe, opérations bit à bit sur entiers.

## Règles générales (à respecter partout)

- **Aucune bibliothèque mathématique**, pas même celle de la std, sauf mention explicite.
- Chaque exercice doit fournir une fonction `main` de test, prête à être compilée/exécutée.
- Complexités vérifiées en soutenance : temps = nombre d'instructions exécutées, espace = mémoire max allouée simultanément, mesurées par rapport à la taille de l'entrée.
- Entrée invalide : comportement indéfini, mais **afficher un message d'erreur** est encouragé (ne pas paniquer en silence).
- Un dossier/binaire par exercice (`ex00` … `ex11`) ; vérifier les noms de fichiers avant rendu.
- Exercices 10 et 11 **non obligatoires**.

## Exercices

Format commun des formules : **notation polonaise inversée (RPN)**, une lettre = un symbole.

| Ex | Fonction | Contraintes |
|----|----------|-------------|
| 00 | `adder(a: u32, b: u32) -> u32` : a + b | Seulement `& \| ^ << >> =` et comparaisons. `++`/`+= 1` uniquement pour l'index de boucle. O(log n) temps et espace. Idée : XOR = somme sans retenue, AND<<1 = retenue, itérer. |
| 01 | `multiplier(a: u32, b: u32) -> u32` : a * b | Mêmes opérateurs interdits/permis. O(1) (32 itérations max). Idée : décalage-addition en réutilisant `adder`. |
| 02 | `gray_code(n: u32) -> u32` | `n ^ (n >> 1)`. Exemples : 0→0, 1→1, 2→3, 3→2, 4→6, 5→7, 6→5, 7→4, 8→12. |
| 03 | `eval_formula(formula: &str) -> bool` | Symboles : `0 1 ! & \| ^ > =`. O(n). Pile pour évaluer. `"10&"`=false, `"10\|"`=true, `"11>"`=true, `"10="`=false, `"1011\|\|="`=true. Conseil du sujet : construire un AST binaire (réutilisable ensuite). |
| 04 | `print_truth_table(formula: &str)` | Variables `A..Z` (≤ 26, réutilisables). O(2^n). Format exact ci-dessous. |
| 05 | `negation_normal_form(formula: &str) -> String` | Sortie uniquement avec variables et `! & \|`; `!` juste après une variable. Plusieurs résultats valides. |
| 06 | `conjunctive_normal_form(formula: &str) -> String` | Sortie `! & \|` seulement; `!` après variable, tous les `&` à la fin. Taille exponentielle acceptée. Peut réutiliser ex05. |
| 07 | `sat(formula: &str) -> bool` | Existe-t-il une affectation rendant la formule vraie ? O(2^n). `AB\|`=true, `AB&`=true, `AA!&`=false, `AA^`=false. Peut réutiliser ex03/04. |
| 08 | `powerset(set: Vec<i32>) -> Vec<Vec<i32>>` | Espace O(2^n). Entrée sans doublons. |
| 09 | `eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32>` | `A` = 1er ensemble, `B` = 2e, etc. Univers = union de tous les ensembles (donc `A!` seul → `[]`). Ordre du résultat sans importance. Nb d'ensembles ≠ nb de variables → erreur. |
| 10 | `map(x: u16, y: u16) -> f64` | Bijection de [0, 2^16−1]² vers une partie de [0;1] (courbe de Z / Lebesgue / Hilbert...). Entrelacement des bits (Morton) sur 32 bits puis division par 2^32−1 ; attention à la précision f64 (32 bits tiennent dans les 53 de mantisse). |
| 11 | `reverse_map(n: f64) -> (u16, u16)` | Inverse exacte de ex10 : `f⁻¹∘f = id` et `f∘f⁻¹ = id`. |

### Format de sortie ex04 (exact, vérifié avec `cat -e`)

Formule `AB&C|` :

```
| A | B | C | = |
|---|---|---|---|
| 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 1 |
...
| 1 | 1 | 1 | 1 |
```

Colonnes = variables en ordre alphabétique, dernière colonne `=` (résultat), lignes dans l'ordre binaire croissant (A = bit de poids fort), chaque ligne finit par `|` puis `\n`.

### Sémantique des opérateurs

`!` NON (unaire) · `&` ET · `|` OU · `^` OU exclusif · `>` implication (A⇒B ≡ ¬A∨B) · `=` équivalence. En RPN les opérandes précèdent l'opérateur (`AB>` = A⇒B, le 1er dépilé est B).

### Règles de réécriture (ex05/06)

- Double négation : ¬¬A ≡ A
- Implication : A⇒B ≡ ¬A ∨ B
- Équivalence : A⇔B ≡ (A⇒B) ∧ (B⇒A)
- XOR : A⊕B ≡ (A∨B) ∧ ¬(A∧B)
- De Morgan : ¬(A∨B) ≡ ¬A∧¬B ; ¬(A∧B) ≡ ¬A∨¬B
- Distributivité (dans les deux sens) : A∧(B∨C) ≡ (A∧B)∨(A∧C) ; A∨(B∧C) ≡ (A∨B)∧(A∨C)

Exemples attendus (un résultat équivalent suffit, mais ceux-ci servent de tests) :
- NNF : `AB&!`→`A!B!|`, `AB|!`→`A!B!&`, `AB>`→`A!B|`, `AB=`→`AB&A!B!&|`, `AB|C&!`→`A!B!&C!|`
- CNF : `ABCD&|&`→`ABC|BD|&&`, `AB|C&`→`AB|C&`, `AB&!C!|`→`A!B!C!||`, `AB|!C!&`→`A!B!C!&&`

## Plan de travail suggéré

1. Choisir/installer le langage, créer un workspace avec un binaire par exercice.
2. Écrire un module commun (parseur RPN → AST, évaluation, affichage) réutilisé par ex03 à ex09 ; ne pas dupliquer.
3. Ajouter des tests (cas du sujet ci-dessus + cas limites : formule vide, pile déséquilibrée, caractère inconnu, `u32::MAX`, 0).
4. Vérifier les complexités et l'interdiction d'opérateurs arithmétiques en ex00/01 (pas de `+ - * /`, pas de `+= 1` hors index de boucle).
5. Commit réguliers, ne rendre que le nécessaire (pas de `target/`, ajouter un `.gitignore`).

## Notes de conduite pour Claude

- L'utilisateur parle français ; répondre en français, code et identifiants en anglais.
- Respecter strictement les prototypes du sujet.
- Ne pas committer ni pousser sans demande explicite.
