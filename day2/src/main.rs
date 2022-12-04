fn main() {
    let input = include_str!("input.txt");
    println!("Part One {}", part1(input));
    println!("Part Two {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input.as_bytes()
        .chunks(4)
        .map(|b| (b[0], b[2]))
        .map(Game::new_part1)
        .map(|g| g.game_score() + g.shape_score() )
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    input
        .as_bytes()
        .chunks(4)
        .map(|b| (b[0], b[2]))
        .map(Game::new_part2)
        .map(|g| g.game_score() + g.shape_score() )
        .sum::<u32>()
}

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

struct Game(Move, Move);

impl Game {
    fn new_part1(bs: (u8, u8)) -> Game {
        let f = |s| match s {
            b'A' | b'X' => Move::Rock,
            b'B' | b'Y' => Move::Paper,
            b'C' | b'Z' => Move::Scissors,
            _ => unreachable!(),
        };
        Game(f(bs.0), f(bs.1))
    }

    fn new_part2(bs: (u8, u8)) -> Game {
        let a = match bs.0 {
            b'A' => Move::Rock,
            b'B' => Move::Paper,
            b'C' => Move::Scissors,
            _ => unreachable!(),
        };

        let b = match (a, bs.1) {
            (Move::Rock, b'X') => Move::Scissors,
            (Move::Rock, b'Y') => Move::Rock,
            (Move::Rock, b'Z') => Move::Paper,

            (Move::Paper, b'X') => Move::Rock,
            (Move::Paper, b'Y') => Move::Paper,
            (Move::Paper, b'Z') => Move::Scissors,

            (Move::Scissors, b'X') => Move::Paper,
            (Move::Scissors, b'Y') => Move::Scissors,
            (Move::Scissors, b'Z') => Move::Rock,
            _ => unreachable!(),
        };

        Game(a, b)
    }

    fn game_score(&self) -> u32 {
        match self {        
            Game(Move::Rock, Move::Paper) => 6,
            Game(Move::Scissors, Move::Rock) => 6,
            Game(Move::Paper, Move::Scissors) => 6,
            
            Game(Move::Rock, Move::Rock) => 3,
            Game(Move::Paper, Move::Paper) => 3,
            Game(Move::Scissors, Move::Scissors) => 3,
    
            Game(Move::Paper, Move::Rock) => 0,
            Game(Move::Rock, Move::Scissors) => 0,
            Game(Move::Scissors, Move::Paper) => 0,
        }
    }

    fn shape_score(&self) -> u32 {
        match self.1 {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}