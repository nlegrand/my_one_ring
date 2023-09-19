pub mod dice {
    use std::fmt;
    #[derive(Clone, Copy)]
    pub enum FeatDice {
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

    #[derive(Clone, Copy)]
    pub enum SuccessDice {
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

    pub const FEAT_DICE: [FeatDice; 12] = [
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
    

    pub const SUCCESS_DICE: [SuccessDice; 6] = [
	SuccessDice::OutlinedNumber(1),
	SuccessDice::OutlinedNumber(2),
	SuccessDice::OutlinedNumber(3),
	SuccessDice::Number(4),
	SuccessDice::Number(5),
	SuccessDice::SuccessIcon,
    ];
}

pub mod roll {
    use rand::Rng;
    use crate::dice as Tor;
    pub fn success_dice() -> Tor::SuccessDice {
        Tor::SUCCESS_DICE[rand::thread_rng().gen_range(0..=5)]
    }
    pub fn feat_dice() -> Tor::FeatDice {
        Tor::FEAT_DICE[rand::thread_rng().gen_range(0..=11)]
    }
    pub fn dice_value(die: Tor::FeatDice) -> u8 {
	match die {
	    Tor::FeatDice::Number(x) => x,
	    Tor::FeatDice::GandalfRune => 100,
	    Tor::FeatDice::EyeofSauron => 0,
	}
    }
    fn best_feat_dice(die1: Tor::FeatDice, die2: Tor::FeatDice) -> Tor::FeatDice {
	println!("Favoured Feat Dice Results: {} and {}", die1, die2);
	let value1 = dice_value(die1);
	let value2 = dice_value(die2);
	if value1 >= value2 {
	    die1
	}
	else {
	    die2
	}
    }

    fn worst_feat_dice(die1: Tor::FeatDice, die2: Tor::FeatDice) -> Tor::FeatDice {
	println!("Favoured Feat Dice Results: {} and {}", die1, die2);
	let value1 = dice_value(die1);
	let value2 = dice_value(die2);
	if value1 <= value2 {
	    die1
	}
	else {
	    die2
	}
    }
    pub fn favoured_feat_dice() -> Tor::FeatDice {
	best_feat_dice(feat_dice(), feat_dice())
    }
    pub fn ill_favoured_feat_dice() -> Tor::FeatDice {
	worst_feat_dice(feat_dice(), feat_dice())
    }
}

pub mod dice_pool {
    use crate::dice as Tor;
    use crate::roll as Roll;
    pub enum Feat {
	Favoured,
	IllFavoured,
	NeitherFavoured,
    }
    pub struct DicePool {
	feat: Feat,
	success_dice: u8,
    }
    impl DicePool {
	fn roll_success_dice(&self) -> Vec<Tor::SuccessDice> {
	    let mut v: Vec<Tor::SuccessDice> = Vec::new();
	    let mut i = self.success_dice ;
	    while i != 0 {
		v.push(Roll::success_dice());
		i -= 1;
	    }
	    v
	}
	pub fn roll(&self) -> (Tor::FeatDice,Option<Tor::FeatDice>, Vec<Tor::SuccessDice>) {
	    match self.feat {
		Feat::NeitherFavoured => (Roll::feat_dice(), None, self.roll_success_dice()),
		_ => (Roll::feat_dice(),Some(Roll::feat_dice()), self.roll_success_dice()),
	    }
	}
    }
}
