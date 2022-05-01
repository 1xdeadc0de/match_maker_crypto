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
	cards: Vec<Card>,
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

	fn cyclic_shift(&mut self, shift: usize) {
		assert_eq!(self.cards.len(), 5);
		self.cards.rotate_left(shift % 5);
	}

	fn decode(&mut self) -> bool {
		let mut opened_deck = self;
		assert_eq!(opened_deck.cards.len(), 5);
		let first = opened_deck.cards.iter().position(|&x| x == Card::Queen).unwrap();
		opened_deck.cyclic_shift(first);

		match &opened_deck.cards {
			cards if cards[1] == Card::Queen || cards[4] == Card::Queen => true,
			_ => false
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::Card::{King, Queen};
	use crate::{Alice, Bob, Deck};

	#[test]
	fn test_cyclic_shift() {
		let mut deck = Deck {
			cards: vec![King, King, King, King, Queen]
		};
		deck.cyclic_shift(2);
		assert_eq!(deck.cards, vec![King, King, Queen, King, King]);
		deck.cyclic_shift(3);
		assert_eq!(deck.cards, vec![King, King, King, King, Queen]);
		assert_ne!(deck.cards, vec![King, Queen, King, King, King]);
	}

	#[test]
	fn test_full_game_var_shifts() {
		let alice = Alice { secret_input: true };
		let bob = Bob { secret_input: true };
		let alice_deck = alice.encode_alice_input();
		let bob_deck = bob.encode_bob_input();

		let mut deck1 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck2 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck3 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck4 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck5 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());

		deck1.cyclic_shift(1);
		deck2.cyclic_shift(2);
		deck3.cyclic_shift(3);
		deck4.cyclic_shift(4);
		deck5.cyclic_shift(5);

		assert_eq!(deck1.decode(), true);
		assert_eq!(deck2.decode(), true);
		assert_eq!(deck3.decode(), true);
		assert_eq!(deck4.decode(), true);
		assert_eq!(deck5.decode(), true);

		let alice = Alice { secret_input: true };
		let bob = Bob { secret_input: false, };

		let alice_deck = alice.encode_alice_input();
		let bob_deck = bob.encode_bob_input();
		let mut deck1 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck2 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck3 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck4 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck5 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());

		deck1.cyclic_shift(1);
		deck2.cyclic_shift(2);
		deck3.cyclic_shift(3);
		deck4.cyclic_shift(4);
		deck5.cyclic_shift(5);

		assert_eq!(deck1.decode(), false);
		assert_eq!(deck2.decode(), false);
		assert_eq!(deck3.decode(), false);
		assert_eq!(deck4.decode(), false);
		assert_eq!(deck5.decode(), false);

		let alice = Alice { secret_input: false };
		let bob = Bob { secret_input: true, };

		let alice_deck = alice.encode_alice_input();
		let bob_deck = bob.encode_bob_input();
		let mut deck1 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck2 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck3 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck4 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck5 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());

		deck1.cyclic_shift(1);
		deck2.cyclic_shift(2);
		deck3.cyclic_shift(3);
		deck4.cyclic_shift(4);
		deck5.cyclic_shift(5);

		assert_eq!(deck1.decode(), false);
		assert_eq!(deck2.decode(), false);
		assert_eq!(deck3.decode(), false);
		assert_eq!(deck4.decode(), false);
		assert_eq!(deck5.decode(), false);

		let alice = Alice { secret_input: false };
		let bob = Bob { secret_input: false, };

		let alice_deck = alice.encode_alice_input();
		let bob_deck = bob.encode_bob_input();
		let mut deck1 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck2 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck3 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck4 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
		let mut deck5 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());

		deck1.cyclic_shift(1);
		deck2.cyclic_shift(2);
		deck3.cyclic_shift(3);
		deck4.cyclic_shift(4);
		deck5.cyclic_shift(5);

		assert_eq!(deck1.decode(), false);
		assert_eq!(deck2.decode(), false);
		assert_eq!(deck3.decode(), false);
		assert_eq!(deck4.decode(), false);
		assert_eq!(deck5.decode(), false);
	}
}
