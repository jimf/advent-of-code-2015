fn main() {
    const INPUT_ROW: usize = 2978;
    const INPUT_COL: usize = 3083;
    let mut row = 1;
    let mut col = 1;
    let mut val: u64 = 20151125;

    while !(row == INPUT_ROW && col == INPUT_COL) {
        val = (val * 252533) % 33554393;
        if row == 1 {
            row = col + 1;
            col = 1;
        } else {
            col += 1;
            row -= 1;
        }
    }

    println!("A: {}", val);
}
