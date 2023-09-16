use my_one_ring::roll as Roll;


fn main() {
    let feat_dice_result = Roll::feat_dice();
    let success_dice_result = Roll::success_dice();
    let favoured_feat_dice_result = Roll::favoured_feat_dice();
    println!("Success dice: {}", success_dice_result);
    println!("Feat dice:{}", feat_dice_result);
    println!("Favoured feat dice:{}", favoured_feat_dice_result);
}
