pub trait SomeTrait {
    /// Retourne un booléen.
    ///
    /// Cette méthode renvoie toujours `true`.
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    /// Retourne un booléen.
    ///
    /// Cette méthode renvoie toujours `true`.
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

/// Fonction `some_func` prenant un type qui implémente `SomeTrait` et `OtherTrait` et renvoyant un booléen.
///
/// # Arguments
///
/// * `item` - Un type qui implémente `SomeTrait` et `OtherTrait`.
///
/// # Returns
///
/// Un booléen indiquant si `item` implémente à la fois `SomeTrait` et `OtherTrait`.
fn some_func<T>(item: T) -> bool
where
    T: SomeTrait + OtherTrait, // Contrainte générique indiquant que T doit implémenter SomeTrait et OtherTrait
{
    item.some_function() && item.other_function() // Appel des méthodes some_function et other_function sur l'objet item
}

fn main() {
    some_func(SomeStruct {}); // Appel de some_func avec une instance de SomeStruct
    some_func(OtherStruct {}); // Appel de some_func avec une instance de OtherStruct
}
