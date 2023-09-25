pub mod dice {
    use std::fmt;
    #[derive(Clone, Copy)]
    pub enum FeatDice {
        Number(i8),
        GandalfRune,
        EyeofSauron,
    }
    impl FeatDice {
        pub fn unpack_values(&self, miserable: bool) -> i8 {
            match self {
                FeatDice::Number(a) => *a,
                FeatDice::GandalfRune => 40,
                FeatDice::EyeofSauron => if miserable { 0 } else { -40 },
            }
        }
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

    #[derive(Debug, Clone, Copy)]
    pub enum SuccessDice {
        OutlinedNumber(i8),
        Number(i8),
        SuccessIcon,
    }
    impl SuccessDice {
        pub fn successes(self) -> i8 {
            match self {
                SuccessDice::SuccessIcon => 1,
                _ => 0,
            }
        }
        pub fn value(&self, weary: bool) -> i8 {
            match self {
                SuccessDice::OutlinedNumber(a) => if weary { 0 } else { *a },
                SuccessDice::Number(a) => *a,
                SuccessDice::SuccessIcon => 6,
            }
        }
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
    pub fn dice_value(die: Tor::FeatDice) -> i8 {
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

pub mod outcome {
    use crate::dice as Tor;
    use crate::dice_pool::Feat as Feat;
    pub struct Raw {
        feat_dice: Tor::FeatDice,
        optional_feat_dice: Option<Tor::FeatDice>,
        feat: Feat,
        successes: Vec<Tor::SuccessDice>,
    }
    struct Computed {
        outcome: i8,
        successes: i8,
    }
    impl Raw {
        fn compute_outcome(&self) -> Computed {
            let mut computed = Computed {
                outcome: 0,
                successes: 0,
            };
            for die in &self.successes {
                computed.successes += die.successes();
            }
            for die in &self.successes {
                computed.outcome += die.value(false); //to be implemented properly
            }
            computed
        }
        
    }
}

pub mod dice_pool {
    use std::fmt;
    use crate::dice as Tor;
    use crate::roll as Roll;
    use crate::outcome::Raw as Outcome;
    #[derive(Clone, Copy)]
    pub enum Feat {
	Favoured,
	IllFavoured,
	Normal,
    }
    impl fmt::Display for Feat {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	    match *self {
	        Feat::Favoured => write!(f, "Favoured"),
	        Feat::IllFavoured => write!(f, "Ill favoured"),
		Feat::Normal => write!(f, "Normal"),
	    }
        }
    }
    pub struct DicePool {
	pub feat: Feat,
	pub success_dice: u8,
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
	pub fn roll(&self) -> (Tor::FeatDice,Option<Tor::FeatDice>, Feat, Vec<Tor::SuccessDice>) {
	    match self.feat {
		Feat::Normal => (Roll::feat_dice(), None, self.feat, self.roll_success_dice()),
		_ => (Roll::feat_dice(),Some(Roll::feat_dice()), self.feat, self.roll_success_dice()),
	    }
	}
    }
}

