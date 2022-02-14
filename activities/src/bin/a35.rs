// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn tile_analyzer(tile: Tile) -> String {
    match tile {
        Tile::Water(Pressure(t)) if t > 10 => "High pressure".to_string(),
        Tile::Water(Pressure(t)) if t < 10 => format!("low water pressure: {} Bar", t).to_string(),
        Tile::Brick(t @ BrickStyle::Gray | t @ BrickStyle::Red)
             => format!("Brickstyle: {:?}", t).to_string(),
        Tile::Brick(t)
             => format!("{:?} type brick", t).to_string(),
        Tile::Dirt | Tile::Grass | Tile::Sand => "ground tile".to_string(),
        Tile::Treasure(TreasureChest {content: TreasureItem::Gold,amount: t }) if t >= 100 => "Lots of gold!".to_string(),
        _ => "different tile".to_string()
    }
}
fn main() {
    let t1 = Tile::Water(Pressure(9));
    let t2 = Tile::Brick(BrickStyle::Dungeon);
    let t3 = Tile::Dirt;
    let t4 = Tile::Treasure(TreasureChest{content: TreasureItem::Gold, amount: 150});



    println!("{:?}", t1);
    println!("{}", tile_analyzer(t1) );
    println!("{}", tile_analyzer(t2) );
    println!("{}", tile_analyzer(t3) );
    println!("{}", tile_analyzer(t4) );

    


}
