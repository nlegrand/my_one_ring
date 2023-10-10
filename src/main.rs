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

    /// Simulate four million roll
    #[arg(long, action = clap::ArgAction::SetTrue)]
    simulation: bool,
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
    if cfg!(debug_assertions) {
        println!("DEBUG feat status: {:?}", feat_status);
    }


    let condition = if cli.weary && cli.miserable {
        MorDice::WEARY_AND_MISERABLE
    }
    else if cli.weary {
        MorDice::WEARY
    }
    else if cli.miserable {
        MorDice::MISERABLE
    }
    else {
        MorDice::NO_CONDITIONS
    };
    if cfg!(debug_assertions) {
        println!("DEBUG condition: {:?}", condition);
    }
    let dp = MorDice::DicePool {
        feat_status: feat_status,
        success_dice: sd,
    };
    if cli.simulation {
        let sim_outcome = dp.simulation(condition);
        if cfg!(debug_assertions) {
            println!("{:?}", sim_outcome);
        }
        sim_outcome.pp();
    }
    else {
        let outcome = dp.roll();
        if cfg!(debug_assertions) {
            println!("DEBUG raw outcome: {:?}", outcome);
        }
        pp_outcome(outcome, condition);
    }
}

fn pp_outcome(outcome: MorDice::Raw, condition: MorDice::Condition) {
    if condition.weary {
        println!("Condition: weary");
    }
    if condition.miserable {
        println!("Condition: miserable");
    }
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
