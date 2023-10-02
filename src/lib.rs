pub mod dice {

    use std::fmt;
    use rand::Rng;

    #[derive(Debug, Clone, Copy)]
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



    pub fn success_dice() -> SuccessDice {
        SUCCESS_DICE[rand::thread_rng().gen_range(0..=5)]
    }
    pub fn feat_dice() -> FeatDice {
        FEAT_DICE[rand::thread_rng().gen_range(0..=11)]
    }
    pub fn dice_value(die: FeatDice) -> i8 {
	match die {
	    FeatDice::Number(x) => x,
	    FeatDice::GandalfRune => 100,
	    FeatDice::EyeofSauron => 0,
	}
    }
    fn best_feat_dice(die1: FeatDice, die2: FeatDice) -> FeatDice {
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

    fn worst_feat_dice(die1: FeatDice, die2: FeatDice) -> FeatDice {
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
    pub fn favoured_feat_dice() -> FeatDice {
	best_feat_dice(feat_dice(), feat_dice())
    }
    pub fn ill_favoured_feat_dice() -> FeatDice {
	worst_feat_dice(feat_dice(), feat_dice())
    }


    #[derive(Debug)]
    pub struct Raw {
        pub feat_dice: FeatDice,
        pub optional_feat_dice: Option<FeatDice>,
        pub feat: Feat,
        pub success_dice: Vec<SuccessDice>,
    }
    #[derive(Debug)]
    pub struct Computed {
        pub automatic_success: bool, //could remove pub if pp where
 // defined as an impl of Computed and not in main.rs
        pub automatic_failure: bool,
        pub outcome: i8,
        pub successes: i8,
    }
    pub struct Condition {
        pub weary: bool,
        pub miserable: bool,
    }
    pub const NO_CONDITIONS: Condition = Condition {
        weary: false,
        miserable: false,
    };
    pub const WEARY: Condition = Condition {
        weary: true,
        miserable: false,
    };
    pub const MISERABLE: Condition = Condition {
        weary: false,
        miserable: true,
    };
    pub const WEARY_AND_MISERABLE: Condition = Condition {
        weary: true,
        miserable: true,
    };
    impl Raw {
	fn pick_feat_dice(&self, feat: Feat, fd1: FeatDice, fd2: FeatDice) -> FeatDice {
	    match feat {
		Feat::Normal => { // should not happen
		    fd1
		},
		Feat::Favoured => {
		    best_feat_dice(fd1, fd2)
		}
		Feat::IllFavoured => {
		    worst_feat_dice(fd1, fd2)
		}
	    }
	}
        pub fn compute(&self, condition: Condition) -> Computed {
            let mut computed = Computed {
                automatic_success: false,
                automatic_failure: false,
                outcome: 0,
                successes: 0,
            };
            for die in &self.success_dice {
                computed.successes += die.successes();
            }
            for die in &self.success_dice {
                computed.outcome += die.value(condition.weary);
            }
            let feat_dice: FeatDice;
            match self.feat {
                Feat::Normal => {
                    feat_dice = self.feat_dice;
                },
                _ => {
		    match self.optional_feat_dice {
			Some(a) => {
			    feat_dice = self.pick_feat_dice(self.feat, self.feat_dice, a)
			},
			None => { //Should not happen so pull a weird result
			    feat_dice = FeatDice::Number(100);
			},
		    }
                } ,
            }
            match feat_dice {
                FeatDice::Number(a) => {
                    computed.outcome += a;
                },
                FeatDice::GandalfRune => {
                    computed.automatic_success = true;
                },
                FeatDice::EyeofSauron => {
                    if condition.miserable {
                        computed.automatic_failure = true;
                    }
                },
            }
        computed
        }
    }


    #[derive(Debug, Clone, Copy)]
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
	fn roll_success_dice(&self) -> Vec<SuccessDice> {
	    let mut v: Vec<SuccessDice> = Vec::new();
	    let mut i = self.success_dice ;
	    while i != 0 {
		v.push(success_dice());
		i -= 1;
	    }
	    v
	}
	pub fn roll(&self) -> Raw {
	    match self.feat {
		Feat::Normal => Raw {
		    feat_dice: feat_dice(),
		    optional_feat_dice: None,
		    feat: self.feat,
		    success_dice: self.roll_success_dice(),
		},
		_ => Raw {
		    feat_dice: feat_dice(),
		    optional_feat_dice: Some(feat_dice()),
		    feat: self.feat,
		    success_dice: self.roll_success_dice(),
		},
	    }
	}
    }
}


