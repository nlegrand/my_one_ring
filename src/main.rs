use the_one_ring_dice::throw as Throw;


fn main() {
    let feat_dice_result = Throw::feat_dice_throw();
    let success_dice_result = Throw::success_dice_throw();
    println!("{}", success_dice_result);
    println!("{}", feat_dice_result);
}
