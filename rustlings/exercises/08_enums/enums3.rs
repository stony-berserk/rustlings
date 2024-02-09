/// Enumération des différents types de message pouvant être traités.
enum Message {
    ChangeColor(u8, u8, u8),
    Echo(String),
    Move(Point),
    Quit,
}

/// Structure représentant une position 
struct Point {
    x: u8,
    y: u8,
}

/// Structure représentant l'état actuel du programme.
struct State {
    /// Couleur actuelle (r, g, b).
    color: (u8, u8, u8),
    /// Position actuelle.
    position: Point,
    /// Indique si le programme doit se terminer.
    quit: bool,
    /// Message actuel.
    message: String,
}

impl State {
    /// Change la couleur de l'état.
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    /// Termine le programme.
    fn quit(&mut self) {
        self.quit = true;
    }

    /// Répète le message fourni.
    /// # Arguments
    /// * `s` - Chaîne de caractères à répéter.
    fn echo(&mut self, s: String) {
        self.message = s;
    }

    /// Déplace le point.
    /// # Arguments
    /// * `p` - Nouvelle position du point.
    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    /// Traite un message donné en mettant à jour l'état en conséquence.
    /// # Arguments
    /// * `message` - Message à traiter.
    /// Solution: match sur le type de message
    fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),
            Message::Echo(s) => self.echo(s),
            Message::Move(p) => self.move_position(p),
            Message::Quit => self.quit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}
