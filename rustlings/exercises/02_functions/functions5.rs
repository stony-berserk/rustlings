/// Ce programme illustre la définition et l'utilisation d'une fonction pour calculer le carré d'un nombre.
///
/// La fonction `main` est le point d'entrée du programme. À l'intérieur de `main`, la fonction `square`
/// est appelée avec l'argument 3, et le résultat est stocké dans la variable `answer`. Ensuite, le résultat
/// est imprimé à l'aide de la macro `println!`.
///
/// La fonction `square` prend un paramètre `num` de type `i32` et retourne un `i32` qui est le carré de `num`.
/// # Arguments
/// * `num` - Un entier de 32 bits dont le carré doit être calculé.
/// 
/// Solution: ajouter le type de retourn de la fonction square

fn main() {
    let answer = square(3); // appel fonction square
    println!("The square of 3 is {}", answer); // print answer
}

/// Calcule le carré d'un nombre entier.
/// # Arguments
/// * `num` - Un entier dont le carré doit être calculé.
/// # Returns
/// Le carré de l'entier passé en argument.
fn square(num: i32) -> i32 { // fonction square prend en param un i32 et return un i32
    num * num // return num² 
}
