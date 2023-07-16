use std::fmt;

enum FeatDice {
    Number(u8),
    GandalfRune,
    EyeofSauron,
}

impl fmt::Display for FeatDice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match *self {
	    FeatDice::Number(i) => write!(f, "{}", i),
	    FeatDice::GandalfRune => write!(f, "Gandalf rune"),
	    FeatDice::EyeofSauron => write!(f, "Eye of Sauron"),
	}
    }
}

enum SuccessDice {
    OutlinedNumber(u8),
    Number(u8),
    SuccessIcon,
}

impl fmt::Display for SuccessDice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match *self {
	    SuccessDice::OutlinedNumber(i) => write!(f, "{}", i),
	    SuccessDice::Number(i) => write!(f, "{}", i),
	    SuccessDice::SuccessIcon => write!(f, "Success icon"),
	}
    }
}

fn main() {
    let feat_dice = vec![
	FeatDice::Number(1),
	FeatDice::Number(2),
	FeatDice::Number(3),
	FeatDice::Number(4),
	FeatDice::Number(5),
	FeatDice::Number(6),
	FeatDice::Number(7),
	FeatDice::Number(8),
	FeatDice::Number(9),
	FeatDice::Number(10),
	FeatDice::GandalfRune,
	FeatDice::EyeofSauron,
    ];

    let success_dice = vec![
	SuccessDice::OutlinedNumber(1),
	SuccessDice::OutlinedNumber(2),
	SuccessDice::OutlinedNumber(3),
	SuccessDice::Number(4),
	SuccessDice::Number(5),
	SuccessDice::SuccessIcon,
	];
    let toto = SuccessDice::Number(4);
    println!("{}", toto);
    println!("{} {}",feat_dice[1], success_dice[2]);
}
