// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * Write a program that uses an iterator to generate a score multiplier
// * The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity

#[derive(Debug)]
struct Gamer {
    score: u32,
    multiplier: u32,
}

impl Gamer {
    fn new() -> Gamer {
        Gamer {
            score: 0,
            multiplier: 0
        }
    }

    fn powerup(&mut self, factor: u32) {
        self.multiplier += factor;
    }
}

impl Iterator for Gamer {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.score += self.multiplier;
        Some(self.score)
    }
}

fn main() {
    let mut game = Gamer::new();
    game.powerup(3);
    println!("{:?}", game);
    println!("{:?}", game.next());
    println!("{:?}", game.next());
    game.powerup(10);

    println!("{:?}", game.next());



}
