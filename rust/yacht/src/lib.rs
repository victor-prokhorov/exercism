#[repr(u8)]
pub enum Category {
    Ones = 1,
    Twos = 2,
    Threes = 3,
    Fours = 4,
    Fives = 5,
    Sixes = 6,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}
use Category::*;
type Dice = [u8; 5];
pub fn score(dice: Dice, category: Category) -> u8 {
    let mut dice = dice;
    dice.sort();
    match category {
        FullHouse => {
            // since dice is sorted if two elements are equal
            // then all of the elements in between
            // are also equal
            if (dice[0] == dice[2] && dice[3] == dice[4]
                || dice[0] == dice[1] && dice[2] == dice[4])
                && dice[0] != dice[4]
            {
                dice.iter().sum()
            } else {
                0
            }
        }
        FourOfAKind => {
            // if either of this this is true
            // then both conditions have overlap
            // so our value is in 1..=3
            if dice[0] == dice[3] || dice[1] == dice[4] {
                4 * dice[1]
            } else {
                0
            }
        }
        LittleStraight => {
            if dice == [1, 2, 3, 4, 5] {
                30
            } else {
                0
            }
        }
        BigStraight => {
            if dice == [2, 3, 4, 5, 6] {
                30
            } else {
                0
            }
        }
        Choice => dice.iter().sum(),
        Yacht => {
            for x in dice.into_iter().skip(1) {
                if dice[0] != x {
                    return 0;
                }
            }
            50
        }
        _ => {
            let c = category as u8;
            dice.iter().filter(|x| **x == c).sum()
        }
    }
}
