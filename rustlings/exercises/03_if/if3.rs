/// Cette fonction détermine l'habitat d'un animal en fonction de son nom.
///
/// # Arguments
/// * `animal` - Le nom de l'animal en tant que chaîne de caractères.
/// # Returns
/// L'habitat de l'animal en tant que chaîne de caractères statique.
/// # Exemple
/// ```
/// let habitat = animal_habitat("snake");
/// assert_eq!(habitat, "Desert");
/// ```
/// 
/// Solution: tous les retours du if doivent être du même type int pour la comparaison dans le if suivant.

pub fn animal_habitat(animal: &str) -> &'static str { // fonction animal_habitat prend en param une str animal et return l'@ d'une str statique
    let identifier = if animal == "crab" { // différents cases
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        0
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}
