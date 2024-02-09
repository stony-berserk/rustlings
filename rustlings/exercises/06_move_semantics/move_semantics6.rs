/// Fonction principale où les démonstrations sont effectuées.
fn main() {
    let data = "Rust is great!".to_string(); // Crée une nouvelle chaîne de caractères et la stocke dans `data`

    get_char(&data); // Appelle la fonction get_char en passant une référence à `data`

    string_uppercase(data); // Appelle la fonction string_uppercase en passant `data` 
}

/// Renvoie le dernier caractère d'une chaîne de caractères donnée.
/// # Arguments
/// * `data` - Une référence à une chaîne de caractères.
/// # Returns
/// Le dernier caractère de la chaîne de caractères.
/// 
/// Solution: ne doit pas prendre la possession donc le type de data doit etre &String

fn get_char(data: &String) -> char {
    data.chars().last().unwrap() // Renvoie le dernier caractère de la chaîne de caractères `data`
}

/// Convertit une chaîne de caractères en majuscules et l'affiche.
/// # Arguments
/// * `data` - Une chaîne de caractères à convertir en majuscules.
///
/// Solution: ne doit pas prendre la possession donc le type de data doit etre &String

fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); // Convertit la chaîne de caractères en majuscules

    println!("{}", data); // Affiche la chaîne de caractères en majuscules
}
