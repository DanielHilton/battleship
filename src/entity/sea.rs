use crate::entity::ship;

#[derive(Copy, Clone)]
enum SeaState {
    Empty,
    Occupied,
    Hit
}

impl SeaState {
    fn print(&self, hide_ships: bool) {
        match self {
            SeaState::Empty => print!(" ~ "),
            SeaState::Hit => print!(" * "),
            SeaState::Occupied => match hide_ships {
                true => print!(" ~ "),
                false => print!(" + "),
            },
        }
    }
}

pub struct Sea {
    spaces: [[SeaState; 10]; 10],
    ships: Vec<ship::Ship>,
}

impl Sea {
    pub fn place_ship(&mut self, mut x: usize, mut y: usize, ship: ship::Ship) {
        let mut count = 0;

        match ship.orientation {
            ship::ShipOrientation::Vertical => {
                while count < ship.length {
                    self.spaces[x][y] = SeaState::Occupied;
                    y += 1;
                    count += 1;
                }
            },
            ship::ShipOrientation::Horizontal => {
                while count < ship.length {
                    self.spaces[x][y] = SeaState::Occupied;
                    x += 1;
                    count += 1;
                }
            }
        }

        self.ships.push(ship);
    }

    pub fn print(&self, hide_ships: bool) {
        let mut x: usize = 0;
        let mut y: usize = 0;
        while x < self.spaces.len() {
            while y < self.spaces[x].len() {
                self.spaces[x][y].print(hide_ships);
                y += 1;
            }

            println!();

            x += 1;
            y = 0;
        }
    }
}

pub fn new_sea() -> Box<Sea> {
    Box::new(Sea { spaces: [[SeaState::Empty; 10]; 10], ships: Vec::new() })
}