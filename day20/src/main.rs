fn elves_and_houses(houses_per_elf: usize, points_per_house: usize) -> usize {
    const INPUT: usize = 33100000;
    const STACK_SIZE: usize = INPUT / 40; // Had to play with this value. INPUT was causing stack overflows.
    let mut points: [usize; STACK_SIZE] = [0; STACK_SIZE];
    let mut elf = 1;
    let mut result: usize = 0;

    loop {
        for house_num in (elf..STACK_SIZE).step_by(elf).take(houses_per_elf) {
            points[house_num] += elf; // Just add elf to keep values smaller.
        }

        if points[elf] >= INPUT / points_per_house { // Divide by 10 since we were only adding elf above.
            result = elf;
            break;
        }

        elf += 1;

        if elf >= STACK_SIZE {
            println!("reached end without solution!");
            break;
        }
    }

    result
}

fn main() {
    println!("A: {}", elves_and_houses(usize::max_value(), 10));
    println!("B: {}", elves_and_houses(50, 11));
}
