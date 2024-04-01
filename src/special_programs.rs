pub mod brain;

pub fn segment_display() -> Vec<u8> {
    let digits: [u8; 10] = [
        0b11100111, // 0
        0b10000100, // 1
        0b11010011, // 2
        0b11010110, // 3
        0b10110100, // 4
        0b01110110, // 5
        0b01110111, // 6
        0b11000100, // 7
        0b11110111, // 8
        0b11110100  // 9
    ];

    let mut result = Vec::<u8>::new();
    for i in 0..=0b11_11111111 as usize {
        let place = (i & 0b11_00000000) >> 8;
        let number = i & 0b00_11111111;

        let hundreds = number / 100;
        let tens = (number / 10) % 10;
        let ones = number % 10;

        match place {
            0 => result.push(0),
            1 => result.push(digits[hundreds]),
            2 => result.push(digits[tens]),
            3 => result.push(digits[ones]),
            _ => panic!("Invalid place")
        }
    }
    return result;
}