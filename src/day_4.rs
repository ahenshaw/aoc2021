const CALLED: usize = 100;

struct Card(Vec<usize>);

impl Card {
    fn add_ball(&mut self, ball: &usize) {
        match self.0.iter().position(|&r| r == *ball) {
            Some(index) => self.0[index] = CALLED,
            _ => (),
        }
    }

    fn is_winner(&self) -> bool {
        let row_winner = self.0.chunks(5).any(|row| row.iter().all(|&x| x==CALLED));
        let mut cols: Vec<_> = (0..5).map(|i| (i..25).step_by(5)).collect();
        let col_winner = cols.iter_mut().any(|col| col.all(|c|self.0[c] == CALLED));
        row_winner || col_winner
    }

    fn score(&self) -> usize {
        self.0.iter().filter(|&&x| x != CALLED).sum()
    }
}

pub struct Input {
    balls: Vec<usize>,
    cards: Vec<Card>,
}

pub fn input_generator(input: &str) -> Input {
    let mut data = input.split("\n\n");
    let balls = data.next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect();
    let cards: Vec<Card> = data.map(|s| Card(s.split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect())).collect();
    Input{balls, cards}
}

pub fn part1(input: &Input) -> usize {
    let balls = input.balls.clone();
    let mut cards: Vec<Card> = input.cards.iter().map(|x| Card(x.0.clone())).collect();

    for ball in balls {
        for card in cards.iter_mut() {
            card.add_ball(&ball);
            if card.is_winner() {return ball * card.score()}
        }
    }
    unreachable!()
}

pub fn part2(_input: &Input) -> usize {
    0
}
