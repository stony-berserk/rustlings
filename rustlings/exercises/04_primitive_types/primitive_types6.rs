/// Ce test démontre comment accéder aux éléments d'un tuple en utilisant la syntaxe d'indexation des tuples.
///
/// Un tuple `numbers` est défini avec trois éléments. Nous devons accéder au deuxième élément.
///
/// Solution: L'expression `numbers.1` est utilisée pour accéder au deuxième élément du tuple.

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3); // init le tuple
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1; // select 2eme index

    assert_eq!(2, second,
        "Ce n'est pas le deuxième nombre dans le tuple !")
}
