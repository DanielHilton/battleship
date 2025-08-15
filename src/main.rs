mod entity;

fn main() {
    println!("Battleship: Rust Edition");

    let mut player_sea = entity::sea::new_sea();
    // let cpu_sea = entity::sea::new_sea();

    player_sea.place_ship(0, 0,
        entity::ship::new(
            entity::ship::ShipKind::AircraftCarrier,
            entity::ship::ShipOrientation::Horizontal)
    );

    player_sea.place_ship(5,9, entity::ship::new(
        entity::ship::ShipKind::Cruiser,
        entity::ship::ShipOrientation::Vertical)
    );

    player_sea.print(false);
    println!("Time to place your ships!")
}
