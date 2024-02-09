/// import SystemTime et UNIX_EPOCH
/// affiche le nombre de secondes écoulées depuis UNIX_EPOCH

// TODO: Complete this use statement
// Importation des modules SystemTime et UNIX_EPOCH depuis le module std::time de la bibliothèque standard Rust
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Utilisation du match pour gérer les résultats de la durée écoulée depuis UNIX_EPOCH
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        // Si la durée depuis UNIX_EPOCH est obtenue avec succès, affiche le nombre de secondes écoulées
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        // En cas d'erreur (par exemple, si le temps système est antérieur à UNIX_EPOCH), déclenche une panique
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

