pub mod dice {

    use std::fmt;
    use rand::Rng;

    #[derive(Debug, Clone, Copy)]
    pub enum FeatDice {
        Number(usize),
        GandalfRune,
        EyeofSauron,
    }
    impl FeatDice {
        pub fn unpack_values(&self) -> usize {
            match self {
                FeatDice::Number(a) => *a,
                FeatDice::GandalfRune => 0,
                FeatDice::EyeofSauron => 0,
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
        OutlinedNumber(usize),
        Number(usize),
        SuccessIcon,
    }
    impl SuccessDice {
        pub fn successes(self) -> usize {
            match self {
                SuccessDice::SuccessIcon => 1,
                _ => 0,
            }
        }
        pub fn value(&self, weary: bool) -> usize {
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
                SuccessDice::SuccessIcon => write!(f, "6 (Success icon)"),
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
    pub fn dice_value(die: FeatDice) -> usize {
        match die {
            FeatDice::Number(x) => x,
            FeatDice::GandalfRune => 100,
            FeatDice::EyeofSauron => 0,
        }
    }
    fn best_feat_dice(die1: FeatDice, die2: FeatDice) -> FeatDice {
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
        pub second_feat_dice: Option<FeatDice>,
        pub feat_status: FeatStatus,
        pub success_dice: Vec<SuccessDice>,
    }
    #[derive(Debug)]
    pub struct Computed {
        pub automatic_success: bool, //could remove pub if pp where
        // defined as an impl of Computed and not in main.rs
        pub automatic_failure: bool,
        pub outcome: usize,
        pub successes: usize,
    }
    #[derive(Debug, Clone)]
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
        fn pick_feat_dice(&self) -> FeatDice {
            match self.second_feat_dice {
                None => {
                    self.feat_dice
                },
                Some(second_feat_dice) => {
                    match self.feat_status {
                        FeatStatus::Favoured =>
                            best_feat_dice(self.feat_dice, second_feat_dice),
                        FeatStatus::IllFavoured =>
                            worst_feat_dice(self.feat_dice, second_feat_dice),
                        FeatStatus::Normal => panic!("Second feat dice present while FeatDiceStatus is Normal")
                    }
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
            let picked_feat_dice = self.pick_feat_dice();
            match picked_feat_dice {
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
    pub enum FeatStatus {
        Favoured,
        IllFavoured,
        Normal,
    }
    impl fmt::Display for FeatStatus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                FeatStatus::Favoured => write!(f, "Favoured"),
                FeatStatus::IllFavoured => write!(f, "Ill favoured"),
                FeatStatus::Normal => write!(f, "Normal"),
            }
        }
    }

    pub struct DicePool {
        pub feat_status: FeatStatus,
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
            match self.feat_status {
                FeatStatus::Normal => Raw {
                    feat_dice: feat_dice(),
                    second_feat_dice: None,
                    feat_status: self.feat_status,
                    success_dice: self.roll_success_dice(),
                },
                _ => Raw {
                    feat_dice: feat_dice(),
                    second_feat_dice: Some(feat_dice()),
                    feat_status: self.feat_status,
                    success_dice: self.roll_success_dice(),
                },
            }
        }
        pub fn simulation(&self, condition: Condition) -> SimulationRow {
            let mut simulation_row = SimulationRow {
                automatic_successes: 0,
                automatic_failures: 0,
                result_count: [0;59],
                successes_count: [0; 9],

            };
            for _i in 0..4000000 {
                let outcome = self.roll();
                let computed = outcome.compute(condition.clone());
                if computed.automatic_failure {
                    simulation_row.automatic_failures += 1 ;
                }
                else if computed.automatic_success {
                    simulation_row.automatic_successes += 1 ;
                }
                else {
                    simulation_row.result_count[computed.outcome] += 1 ;
                }
                simulation_row.successes_count[computed.successes] += 1;
            }
            simulation_row
        }
    }
    #[derive(Debug)]
    pub struct SimulationRow {
        pub automatic_successes: u32,
        pub automatic_failures: u32,
        pub result_count: [u32; 59],
        pub successes_count: [u32; 9],
    }

    impl SimulationRow {
        pub fn pp(&self) {
            println!("Automatic successes: {} %", self.automatic_successes as f64 / 40000.0);
            if self.automatic_failures > 0 {
                println!("Automatic failures: {} %", self.automatic_failures as f64 / 40000.0);
            }
            println!("Successes:");
            for (i, el) in self.successes_count.iter().enumerate() {
                if *el > 0 {
                    println!("    {}: {} %", i, *el as f64 / 40000.0);
                }
            }
            let mut cumulative_res: u32 = self.automatic_successes;
            println!("Cumulative results (automatic successes already counted in):");
            for (i, el) in self.result_count.iter().enumerate().rev() {
                cumulative_res += el;
                if i == 19 {
                    println!("    {}: {} %", i, cumulative_res as f64 / 40000.0);
                }
                if i == 18 {
                    println!("    {}: {} %", i, cumulative_res as f64 / 40000.0);
                }
                if i == 17 {
                    println!("    {}: {} %", i, cumulative_res as f64 / 40000.0);
                }
                if i == 16 {
                    println!("    {}: {} %", i, cumulative_res as f64 / 40000.0);
                }
                if i == 15 {
                    println!("    {}: {} %", i, cumulative_res as f64 / 40000.0);
                }
                if i == 14 {
                    println!("    {}: {} %", i, cumulative_res as f64 / 40000.0);
                }
                if i == 13 {
                    println!("    {}: {} %", i, cumulative_res as f64 / 40000.0);
                }
                if i == 12 {
                    println!("    {}: {} %", i, cumulative_res as f64 / 40000.0);
                }
                if i == 11 {
                    println!("    {}: {} %", i, cumulative_res as f64 / 40000.0);
                }
            }
        }
    }
}
