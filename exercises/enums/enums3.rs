// enums3.rs
// Address all the TODOs to make the tests pass!

enum Message {
    // TODO: implement the message variant types based on their usage below
    Quit,
    Echo(String),
    Move(Point),
    ChangeColor((u8, u8, u8)),
    None,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        let mut count: u8 = 1;
        // TODO: create a match expression to process the different message variants
        match message {
            // color, statement, position := representation parameters
            Message::ChangeColor(color) => self.change_color(color),
            Message::Echo(statement) => self.echo(statement),
            Message::Move(position) => self.move_position(position),
            Message::Quit => self.quit(),
            _ => count += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let num = 1;
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
        };
        state.process(Message::ChangeColor((255, 0, 255)));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        println!("{:?}", state.process(Message::None));

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }
}
