/// Importe la structure de données `HashMap` depuis le module `std::collections`.
use std::collections::HashMap;

/// Structure pour stocker les détails des buts d'une équipe.
struct Team {
    name: String,       // Nom de l'équipe
    goals_scored: u8,   // Buts marqués par l'équipe
    goals_conceded: u8, // Buts encaissés par l'équipe
}

/// Construit une table de scores à partir de la chaîne de résultats des matchs.
///
/// # Arguments
///
/// * `results` - Chaîne de résultats des matchs sous forme de texte.
///
/// # Returns
///
/// Une `HashMap` contenant les détails des buts pour chaque équipe.
fn build_scores_table(results: String) -> HashMap<String, Team> {
    // Création d'une nouvelle `HashMap` pour stocker les équipes et leurs détails de buts
    let mut scores: HashMap<String, Team> = HashMap::new();

    // Parcours de chaque ligne de la chaîne de résultats
    for r in results.lines() {
        // Séparation des éléments de la ligne en utilisant la virgule comme délimiteur
        let v: Vec<&str> = r.split(',').collect();

        // Extraction des noms des équipes et de leurs scores à partir de la ligne
        let team_1_name = v[0].to_string(); // Nom de la première équipe
        let team_1_score: u8 = v[2].parse().unwrap(); // Buts marqués par la première équipe
        let team_2_name = v[1].to_string(); // Nom de la deuxième équipe
        let team_2_score: u8 = v[3].parse().unwrap(); // Buts marqués par la deuxième équipe

        // Mise à jour des détails des buts pour la première équipe
        let t1 = scores.entry(team_1_name.clone()).or_insert(Team{
            name: team_1_name.clone(), // Clonage pour éviter de déplacer la propriété
            goals_scored: 0,
            goals_conceded: 0,
        });
        (*t1).goals_scored += team_1_score; // Ajout des buts marqués par la première équipe (accede a la valeur pointer par t1)
        (*t1).goals_conceded += team_2_score; // Ajout des buts encaissés par la première équipe

        // Mise à jour des détails des buts pour la deuxième équipe
        let t2 = scores.entry(team_2_name.clone()).or_insert(Team{
            name: team_2_name.clone(), // Clonage pour éviter de déplacer la propriété
            goals_scored: 0,
            goals_conceded: 0,
        });
        (*t2).goals_conceded += team_1_score; // Ajout des buts encaissés par la deuxième équipe
        (*t2).goals_scored += team_2_score; // Ajout des buts marqués par la deuxième équipe
    }

    // Retourne la table de scores construite
    scores
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
