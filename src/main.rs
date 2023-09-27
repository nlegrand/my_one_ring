use my_one_ring::roll as Roll;
use my_one_ring::dice_pool::Feat as Feat;
use my_one_ring::dice_pool::DicePool as DicePool;


fn main() {
    let feat_dice_result = Roll::feat_dice();
    let success_dice_result = Roll::success_dice();
    let favoured_feat_dice_result = Roll::favoured_feat_dice();
    let ill_favoured_feat_dice_result = Roll::ill_favoured_feat_dice();
    println!("Success dice: {}", success_dice_result);
    println!("Feat dice:{}", feat_dice_result);
    println!("Favoured feat dice:{}", favoured_feat_dice_result);
    println!("Ill favoured feat dice:{}", ill_favoured_feat_dice_result);
    let dp = DicePool {
	feat: Feat::Normal,
	success_dice: 3,
    };
    let outcome = dp.roll();
    println!("my outcome: {:?}", outcome);
    let dpf = DicePool {
	feat: Feat::Favoured,
	success_dice: 2,
    };
    let computed_result = outcome.compute(false);
    println!("my computed result: {:?}", computed_result);
    let favoured_outcome = dpf.roll();
    println!("my favoured outcome: {:?}", favoured_outcome);
}
