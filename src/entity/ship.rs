pub enum ShipKind {
    Destroyer,
    Submarine,
    Cruiser,
    Battleship,
    AircraftCarrier,
}

pub enum ShipOrientation {
    Vertical,
    Horizontal,
}

pub struct Ship {
    pub length: i32,
    pub kind: ShipKind,
    pub orientation: ShipOrientation,
}

pub fn new(kind: ShipKind, orientation: ShipOrientation) -> Ship {
    let length = match kind {
        ShipKind::Destroyer => 2,
        ShipKind::Battleship | ShipKind::Submarine => 3,
        ShipKind::Cruiser => 4,
        ShipKind::AircraftCarrier => 5,
    };

    Ship {
        kind,
        length,
        orientation,
    }
}
