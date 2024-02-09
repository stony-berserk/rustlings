/// Définition d'une structure `Rectangle` représentant un rectangle avec une largeur et une hauteur.
struct Rectangle {
    /// Largeur du rectangle.
    width: i32,
    /// Hauteur du rectangle.
    height: i32,
}

impl Rectangle {
    /// Crée une nouvelle instance de `Rectangle` avec une largeur et une hauteur données.
    ///
    /// # Arguments
    ///
    /// * `width` - La largeur du rectangle.
    /// * `height` - La hauteur du rectangle.
    ///
    /// # Panics
    ///
    /// Panique si `width` ou `height` est négatif ou nul.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::Rectangle;
    ///
    /// let rect = Rectangle::new(10, 20);
    /// assert_eq!(rect.width, 10);
    /// assert_eq!(rect.height, 20);
    /// ```
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Teste si la largeur et la hauteur du rectangle sont correctes.
    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }

    /// Teste si le programme panique lors de la création d'un rectangle avec une largeur négative.
    #[test]
    #[should_panic]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    /// Teste si le programme panique lors de la création d'un rectangle avec une hauteur négative.
    #[test]
    #[should_panic]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
