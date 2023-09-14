use my_one_ring::roll as Roll;


fn main() {
    let feat_dice_result = Roll::feat_dice();
    let success_dice_result = Roll::success_dice();
    println!("{}", success_dice_result);
    println!("{}", feat_dice_result);
}
