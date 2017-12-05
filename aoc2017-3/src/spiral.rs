/*
 * I mostly did this with pencil and paper. I determined that it would be
 * sufficient to only consider the center values of each face of each `ring`
 * of the spiral. The Manhattan distance from these center values is equal to
 * the ring number (if we consider the kernel `1` to be ring 0 lol).
 *
 * I determined the algorithm for identifying centers by examining rings 0-4.
 *
 * This prints out the center values of each ring in the spiral. I visually
 * identified that my input `312051` lies in spiral `279`, closest to center
 * `312202`. Then I compute the distance by hand:
 *
 * (312202 - 312051) + 279 = 430
 */

pub fn spiral() -> () {
    let mut cur: u32 = 1;

    for i in 1.. {
        println!("i = {}", i);
        for j in 1..5 { // Upper bound is non-inclusive, this is 1 through 4
            if j == 1 {
                cur = cur + 2 * i - 1;
            }
            else {
                cur = cur + 2 * i;
            }
            print!("{} ", cur);
        }
        println!("");
        println!("================================================================");
    }
}
