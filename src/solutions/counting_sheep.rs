//TODO
//Consider an array/list of sheep where some sheep may be missing from their place.
//We need a function that counts the number of sheep present in the array (true means present).

pub fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().fold(0, |mut x, y| {
        if *y {
            x += 1;
        }
        x
    })
}
