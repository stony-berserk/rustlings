/// Illustration de l'utilisation du motif de correspondance (`match`) avec les valeurs de type `Option`.
/// La variable `y` de type `Option<Point>` est définie avec une valeur `Some(Point { x: 100, y: 200 })`.
/// En utilisant `match`, nous vérifions si `y` est `Some`. Si c'est le cas, nous affichons les coordonnées `x` et `y` de `Point`.
/// Sinon, si `y` est `None`, nous déclenchons une panique avec un message d'erreur. La variable `y` est utilisée pour éviter un avertissement de variable inutilisée.

// Définition d'une structure Point avec des coordonnées x et y de type i32
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // Déclaration d'une variable y de type Option<Point> contenant Some(Point { x: 100, y: 200 })
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // Utilisation d'un pattern match pour faire correspondre la valeur de y
    match &y {
        // Si y est Some, extrait le Point p et affiche ses coordonnées x et y
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        // Si y est None, déclenche une panique avec un message d'erreur
        _ => panic!("no match!"),
    }

    // Utilisation de la variable y pour éviter un avertissement de variable inutilisée
    y; // Fix without deleting this line.
}
