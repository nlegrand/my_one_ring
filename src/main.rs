use my_one_ring::dice as MorDice;

use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// How many success dice
    #[arg(short, long, value_name = "DICE")]
    success_dice: u8,

    /// The roll is favoured
    #[arg(short, long, action = clap::ArgAction::SetTrue, group="feat_status")]
    favoured: bool,

    /// The roll is ill-favoured
    #[arg(short, long, action = clap::ArgAction::SetTrue, group="feat_status")]
    ill_favoured: bool,

    /// The character is weary
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    weary: bool,

    /// The character is miserable
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    miserable: bool,

}


fn main() {

    let cli = Cli::parse();
    let sd = cli.success_dice;
    let feat_status = if cli.favoured {
        MorDice::FeatStatus::Favoured
    }
    else if cli.ill_favoured {
        MorDice::FeatStatus::IllFavoured
    }
    else {
        MorDice::FeatStatus::Normal
    };

    let dp = MorDice::DicePool {
	feat_status: feat_status,
	success_dice: sd,
    };
    let outcome = dp.roll();
    if cfg!(debug_assertions) {
        println!("Debug, raw outcome: {:?}", outcome);
    }
    pp_outcome(outcome, MorDice::NO_CONDITIONS);
}

fn pp_outcome(outcome: MorDice::Raw, condition: MorDice::Condition) {
    match outcome.feat_status {
	MorDice::FeatStatus::Normal => {
	    println!("Feat dice: {}", outcome.feat_dice);
	},
	_ => {
	    let o_feat_dice = match outcome.second_feat_dice {
		Some(a) => a,
		None => panic!("Favoured or ill-favoured roles should have a second feat dice"),
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
