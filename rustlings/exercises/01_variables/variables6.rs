/// Ce programme illustre l'utilisation des variables en Rust.
///
/// La constante `NUMBER` est déclarée avec le type `i32` et est initialisée à 3.
/// Ensuite, la fonction `main` est définie comme point d'entrée du programme.
/// À l'intérieur de la fonction `main`, le nombre `NUMBER` est print à l'aide de la macro `println!`.
/// 
/// Solution: ajouter le type de NUMBER

const NUMBER:i32 = 3;   // déclaration d'une variable constante/globale NUMBER
fn main() {             // fonction main          
    println!("Number {}", NUMBER);  // print NUMBER
}
