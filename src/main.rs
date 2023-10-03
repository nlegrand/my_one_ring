use my_one_ring::dice as MorDice;

use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// How many success dice
    #[arg(short, long, value_name = "DICE")]
    success_dice: u8,

    /// The roll is favoured
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    favoured: bool,

    /// The roll is ill-favoured
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    ill_favoured: bool,

    /// The character is weary
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    weary: bool,

    /// The character is miserable
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    miserable: bool,

}


fn main() {
    let feat_dice_result = MorDice::feat_dice();
    let success_dice_result = MorDice::success_dice();
    let favoured_feat_dice_result = MorDice::favoured_feat_dice();
    let ill_favoured_feat_dice_result = MorDice::ill_favoured_feat_dice();
    println!("Success dice: {}", success_dice_result);
    println!("Feat dice:{}", feat_dice_result);
    println!("Favoured feat dice:{}", favoured_feat_dice_result);
    println!("Ill favoured feat dice:{}", ill_favoured_feat_dice_result);
    let dp = MorDice::DicePool {
	feat_status: MorDice::FeatStatus::Normal,
	success_dice: 3,
    };
    let outcome = dp.roll();
    println!("my outcome: {:?}", outcome);
    let dpf = MorDice::DicePool {
	feat_status: MorDice::FeatStatus::Favoured,
	success_dice: 2,
    };
    let computed_result = outcome.compute(MorDice::WEARY_AND_MISERABLE);
    println!("my computed result: {:?}", computed_result);
    let favoured_outcome = dpf.roll();
    println!("my favoured outcome: {:?}", favoured_outcome);

    let cli = Cli::parse();

    let sd = cli.success_dice;

    let dp2 = MorDice::DicePool {
	feat_status: MorDice::FeatStatus::Normal,
	success_dice: sd,
    };
    let outcome2 = dp2.roll();
    println!("my outcome2: {:?}", outcome2);
    pp_outcome(favoured_outcome, MorDice::NO_CONDITIONS);
}

fn pp_outcome(outcome: MorDice::Raw, condition: MorDice::Condition) {
    match outcome.feat_status {
	MorDice::FeatStatus::Normal => {
	    println!("Feat dice: {}", outcome.feat_dice);
	},
	_ => {
	    let o_feat_dice = match outcome.second_feat_dice {
		Some(a) => a,
		None => MorDice::FeatDice::Number(100), // should not
 // happen the unusually high number should suggest something fishy is
 // happening.
	    };
	    println!("Feat dice ({}): {}, {}", outcome.feat_status, outcome.feat_dice, o_feat_dice)
	},
    }
    print!("Success dice: ");
    let mut success_dice = outcome.success_dice.clone().into_iter().peekable();
    while let Some(die) = success_dice.next()  {
	if success_dice.peek().is_none() {
            println!("{}.", die);
	}
	else {
	    print!("{}, ", die);
	}
    }
    let computed = outcome.compute(condition);
    if computed.automatic_success {
	println!("Automatic success!!!");
    }
    if computed.automatic_failure {
	println!("Automatic failure!!!");
    }
    println!("The result is {} with {} success(es)", computed.outcome, computed.successes);
}
