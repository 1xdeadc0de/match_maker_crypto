fn main() {
	println!("Hello, world!");
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Card {
	King,
	Queen,
}

#[derive(Clone, PartialEq)]
struct Deck {
	cards: Vec<Card>
}

struct Alice {
	secret_input: bool,
}

struct Bob {
	secret_input: bool,
}

impl Alice {
	fn encode_alice_input(self) -> Deck {
		match self.secret_input {
			true => Deck {
				cards: vec![Card::Queen, Card::King],
			},
			false => Deck {
				cards: vec![Card::King, Card::Queen],
			},
		}
	}
}

impl Bob {
	fn encode_bob_input(self) -> Deck {
		match self.secret_input {
			true => Deck {
				cards: vec![Card::King, Card::Queen],
			},
			false => Deck {
				cards: vec![Card::Queen, Card::King],
			},
		}
	}
}

impl Deck {
	fn join_decks(deck_alice: Self, deck_bob: Self) -> Self {
		let mut new_deck = deck_alice;
		new_deck.cards.extend(vec![Card::King]);
		new_deck.cards.extend(deck_bob.cards);
		new_deck
	}

	fn cycle_shift(&mut self, shift: usize) {
		assert_eq!(self.cards.len(), 5);
		self.cards.rotate_left(shift % 5);
	}

	fn decode(&mut self) -> bool {
		let mut opened_deck = self;
		assert_eq!(opened_deck.cards.len(), 5);
		let first = opened_deck.cards.iter().position(|&x| x == Card::Queen).unwrap();
		opened_deck.cycle_shift(first);

		match &opened_deck.cards {
			cards if cards[1] == Card::Queen || cards[4] == Card::Queen => true,
			_ => false
		}
	}
}
