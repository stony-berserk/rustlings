/// Définition d'une structure `Book` avec des références à des chaînes de caractères de durée de vie `'a`.
struct Book<'a> {
    /// Référence à une chaîne de caractères contenant le nom de l'auteur.
    author: &'a str,
    /// Référence à une chaîne de caractères contenant le titre du livre.
    title: &'a str,
}

fn main() {
    /// Création d'une nouvelle chaîne de caractères contenant le nom de l'auteur.
    let name = String::from("Jill Smith");
    /// Création d'une nouvelle chaîne de caractères contenant le titre du livre.
    let title = String::from("Fish Flying");
    let book = Book {
        /// Référence à la chaîne de caractères 'name' contenant le nom de l'auteur.
        author: &name,
        /// Référence à la chaîne de caractères 'title' contenant le titre du livre.
        title: &title,
    };

    // Affichage du titre et de l'auteur du livre
    println!("{} by {}", book.title, book.author);
}
