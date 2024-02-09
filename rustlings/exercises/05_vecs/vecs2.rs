/// Cette fonction itère sur chaque élément mutable dans le vecteur `v` et multiplie chaque nombre par 2.
/// # Arguments
/// * `v` - Un vecteur d'entiers où chaque élément doit être multiplié par 2.
/// # Returns
/// Un nouveau vecteur contenant les nombres d'origine, mais multipliés par 2.
///
/// # Exemple
/// ```
/// let v: Vec<i32> = vec![1, 2, 3, 4, 5];
/// let modified_v = vec_loop(v);
/// assert_eq!(modified_v, vec![2, 4, 6, 8, 10]);
/// ```
/// Solution: bien référencer element dans les 2 fonctions 

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // Itère sur chaque élément mutable dans le vecteur `v`
        *element = *element * 2; // Multiplie la valeur de l'élément par 2
    }

    // À ce stade, `v` devrait être égal à [4, 8, 12, 16, 20].
    v // Renvoie le vecteur modifié
}

/// Cette fonction utilise la fonction `map` pour multiplier chaque élément du vecteur `v` par 2.
/// # Arguments
/// * `v` - Un vecteur d'entiers où chaque élément doit être multiplié par 2.
/// # Returns
/// Un nouveau vecteur contenant les nombres d'origine, mais multipliés par 2.
/// 
/// # Exemple
/// ```
/// let v: Vec<i32> = vec![1, 2, 3, 4, 5];
/// let modified_v = vec_map(&v);
/// assert_eq!(modified_v, vec![2, 4, 6, 8, 10]);
/// ```
fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // Utilise la fonction map pour appliquer une transformation à chaque élément du vecteur
        *element * 2 // Multiplie chaque élément par 2 et retourne le résultat
    }).collect() // Collecte les résultats transformés dans un nouveau vecteur
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
