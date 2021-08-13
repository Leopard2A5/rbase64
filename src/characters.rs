use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CHARS: &'static [char; 64] = &[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '+', '/'
    ];
}

lazy_static! {
    pub static ref VALUES_BY_CHAR: HashMap<char, u8> = {
        let mut m = HashMap::new();
        CHARS.iter()
            .enumerate()
            .for_each(|(value, chr)| { m.insert(*chr, value as u8); });
        m
    };
}
