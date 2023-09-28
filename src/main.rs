use my_one_ring::dice as MorDice;

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
	feat: MorDice::Feat::Normal,
	success_dice: 3,
    };
    let outcome = dp.roll();
    println!("my outcome: {:?}", outcome);
    let dpf = MorDice::DicePool {
	feat: MorDice::Feat::Favoured,
	success_dice: 2,
    };
    let computed_result = outcome.compute(MorDice::WEARY_AND_MISERABLE);
    println!("my computed result: {:?}", computed_result);
    let favoured_outcome = dpf.roll();
    println!("my favoured outcome: {:?}", favoured_outcome);
}
