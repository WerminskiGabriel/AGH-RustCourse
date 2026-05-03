enum Chessman {
    Pawn { position: Position, color: Color },
    Knight { position: Position, color: Color },
    Bishop { position: Position, color: Color },
    Rook { position: Position, color: Color },
    Queen { position: Position, color: Color },
    King { position: Position, color: Color },
}
#[derive(PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug)]
struct Position {
    x: u8,
    y: u8,
}

impl Chessman {
    fn make_move(&mut self, new_position: Position) -> bool {
        if new_position.x > 7 || new_position.y > 7 {
            return false;
        }

        let is_valid: bool = match self {
            Chessman::Pawn { position, color } => {
                let vector: i16 = if *color == Color::White { 1 } else { -1 };

                let is_valid = position.x == new_position.x
                    && position.y as i16 + vector == new_position.y as i16;

                if is_valid {
                    *position = new_position;
                }
                is_valid
            }

            Chessman::King { position, color } => {
                let dx = (position.x as i16 - new_position.x as i16).abs();
                let dy = (position.y as i16 - new_position.y as i16).abs();

                let is_valid = dx <= 1 && dy <= 1 && (dx != 0 || dy != 0);

                if is_valid {
                    *position = new_position;
                }
                is_valid
            }

            Chessman::Queen { position, color } => {
                let dx = (position.x as i16 - new_position.x as i16).abs();
                let dy = (position.y as i16 - new_position.y as i16).abs();

                let is_valid =
                    (dx == dy && dx != 0) || (dx == 0 && dy != 0) || (dy == 0 && dx != 0);

                if is_valid {
                    *position = new_position;
                }
                is_valid
            }

            Chessman::Rook { position, color } => {
                let dx = (position.x as i16 - new_position.x as i16).abs();
                let dy = (position.y as i16 - new_position.y as i16).abs();

                let is_valid = (dx == 0 && dy != 0) || (dy == 0 && dx != 0);

                if is_valid {
                    *position = new_position;
                }
                is_valid
            }

            Chessman::Knight { position, color } => {
                let dx = (position.x as i16 - new_position.x as i16).abs();
                let dy = (position.y as i16 - new_position.y as i16).abs();

                let is_valid = (dx == 2 && dy == 1) || (dx == 1 && dy == 2);

                if is_valid {
                    *position = new_position;
                }
                is_valid
            }

            Chessman::Bishop { position, color } => {
                let dx = (position.x as i16 - new_position.x as i16).abs();
                let dy = (position.y as i16 - new_position.y as i16).abs();

                let is_valid = (dx == dy) && dx != 0;

                if is_valid {
                    *position = new_position;
                }
                is_valid
            }
        };

        is_valid
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_move() {
        let mut pawn = Chessman::Pawn {
            position: Position { x: 1, y: 2 },
            color: Color::White,
        };
        assert_eq!(pawn.make_move(Position { x: 1, y: 3 }), true);
        assert_eq!(pawn.make_move(Position { x: 1, y: 5 }), false);
    }
}

fn main() {}
