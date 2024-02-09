use std::num::ParseIntError; // Importation du type d'erreur ParseIntError depuis le module std::num

/// Enumération des erreurs possibles lors de la création d'un entier positif non nul.
#[derive(PartialEq, Debug)]
enum CreationError {
    /// La valeur est négative.
    Negative,
    /// La valeur est zéro.
    Zero,
}

/// Enumération des erreurs possibles lors de la conversion d'une chaîne en un entier positif non nul.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    /// Erreur de création d'un entier positif non nul.
    Creation(CreationError),
    /// Erreur de conversion de chaîne en entier.
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    /// Convertit une erreur de création en erreur de conversion personnalisée.
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }

    /// Convertit une erreur de conversion de chaîne en entier en erreur de conversion personnalisée.
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

/// Analyse une chaîne de caractères et renvoie un résultat contenant un entier positif non nul.
///
/// # Arguments
///
/// * `s` - Chaîne de caractères à analyser.
///
/// # Returns
///
/// Un résultat contenant un entier positif non nul ou une erreur en cas d'échec de l'analyse.
fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // Tente de convertir la chaîne en un entier i64 et renvoie une erreur appropriée en cas d'échec.
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

/// Structure représentant un entier positif non nul.
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    /// Crée un nouvel entier positif non nul.
    ///
    /// # Arguments
    ///
    /// * `value` - Valeur de l'entier à créer.
    ///
    /// # Returns
    ///
    /// Un résultat contenant un entier positif non nul ou une erreur si la valeur est négative ou zéro.
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Teste si une erreur de conversion de chaîne en entier est correctement gérée.
    #[test]
    fn test_parse_error() {
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    /// Teste la gestion d'une valeur négative.
    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    /// Teste la gestion d'une valeur zéro.
    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    /// Teste la création d'un entier positif non nul.
    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
