/// Structure générique `Wrapper` avec un type générique `T`.
///
/// Cette structure enveloppe une valeur de type `T`.
struct Wrapper<T> {
    value: T, // Déclaration d'un champ value de type T dans la structure Wrapper
}

impl<T> Wrapper<T> {
    /// Crée une nouvelle instance de `Wrapper` avec une valeur donnée.
    ///
    /// # Arguments
    ///
    /// * `value` - Valeur à envelopper dans le `Wrapper`.
    ///
    /// # Returns
    ///
    /// Une nouvelle instance de `Wrapper` avec la valeur donnée.
    pub fn new(value: T) -> Self {
        // Création et initialisation d'une nouvelle instance de Wrapper avec la valeur donnée
        Wrapper { value } // Retourne l'instance nouvellement créée
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Teste le stockage d'un `u32` dans un `Wrapper`.
    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    /// Teste le stockage d'une chaîne de caractères dans un `Wrapper`.
    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
