// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    let thirteen: Option<u16> = Some(13);
    let ninetynine: Option<u16> = Some(99);

    print_number(thirteen);
    print_number(ninetynine);

    let mut numbers: [Option<u16>; 5] = [None, None, None, None, None];
    for iter in 0..5 {
        let number_to_add: u16 = { ((iter * 1235) + 2) / (4 * 16) };

        numbers[iter as usize] = Some(number_to_add);
    }
}
