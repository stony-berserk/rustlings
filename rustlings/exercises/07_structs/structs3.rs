/// Structure représentant un colis.
#[derive(Debug)] 
struct Package {
    /// Pays de l'expéditeur.
    sender_country: String,
    /// Pays du destinataire.
    recipient_country: String,
    /// Poids en grammes du colis.
    weight_in_grams: u32,
}

impl Package {
    /// Crée un nouveau colis avec les données spécifiées.
    ///
    /// # Arguments
    ///
    /// * `sender_country` - Pays de l'expéditeur.
    /// * `recipient_country` - Pays du destinataire.
    /// * `weight_in_grams` - Poids en grammes du colis.
    ///
    /// # Panics
    ///
    /// Cette fonction panique si le poids du colis est inférieur à 10 grammes.
    /// retourne la structure créée
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package {
        if weight_in_grams < 10 {
            panic!("Can not ship a package with weight below 10 grams.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    /// Vérifie si le colis est international.
    /// Returns `true` si le colis est international (le pays de l'expéditeur est différent du pays du destinataire), sinon `false`.
    fn is_international(&self) -> bool {
        self.recipient_country != self.sender_country
    }

    /// Calcule les frais d'expédition en fonction du tarif par gramme.
    ///
    /// # Arguments
    /// * `cents_per_gram` - Le tarif par gramme pour le calcul des frais d'expédition.
    /// # Returns
    /// Les frais d'expédition pour le colis.
    fn get_fees(&self, cents_per_gram: u32) -> u32 {
       self.weight_in_grams * cents_per_gram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
