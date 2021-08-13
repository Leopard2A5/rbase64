extern crate lazy_static;

mod characters;

use std::io::{Read, BufReader, Error};
use characters::{CHARS, VALUES_BY_CHAR};

pub fn encode<I: Read>(input: I) -> Result<String, Error> {
    let mut reader = BufReader::new(input);

    let mut buf = [0u8; 3];
    let mut result = String::new();

    loop {
        buf.fill(0);
        let read = reader.read(&mut buf)?;

        if read == 0 {
            break;
        }

        let sextet0 = buf[0] >> 2;
        result.push(CHARS[sextet0 as usize]);

        let sextet1 = buf[1] >> 4;
        let sextet1 = sextet1 + ((buf[0] & 0b00000011) << 4);
        result.push(CHARS[sextet1 as usize]);

        if read > 1 {
            let part1 = (buf[1] & 0b00001111) << 2;
            let part2 = buf[2] >> 6;
            let sextet2 = part1 + part2;
            result.push(CHARS[sextet2 as usize]);
        }
        if read > 2 {
            let sextet3 = buf[2] & 0b00111111;
            result.push(CHARS[sextet3 as usize]);
        }

        if read < 3 {
            result.push('=');
        }
        if read < 2 {
            result.push('=');
        }

        if read < 3 {
            break;
        }
    }

    return Ok(result);
}

pub fn decode<I: Read>(input: I) -> Result<Vec<u8>, Error> {
    let mut reader = BufReader::new(input);

    let mut buf = [0u8; 4];
    let mut result = vec![];

    loop {
        buf.fill(0);
        let read = {
            let mut tmp = reader.read(&mut buf)?;

            if buf[3] as char == '=' {
                buf[3] = 0;
                tmp -= 1;
            }
            if buf[2] as char == '=' {
                buf[2] = 0;
                tmp -= 1;
            }

            tmp
        };

        if read == 0 {
            break;
        }

        let chr0 = VALUES_BY_CHAR.get( &(buf[0] as char)).unwrap();
        let chr1 = VALUES_BY_CHAR.get( &(buf[1] as char)).unwrap();

        let part1 = (chr0 & 0b00111111) << 2;
        let part2 = chr1 >> 4;
        result.push(part1 + part2);

        if read > 2 {
            let chr2 = VALUES_BY_CHAR.get( &(buf[2] as char)).unwrap();

            let part1 = (chr1 & 0b00001111) << 4;
            let part2 = (chr2 & 0b00111100) >> 2;
            result.push(part1 + part2);

            if read > 3 {
                let chr3 = VALUES_BY_CHAR.get( &(buf[3] as char)).unwrap();
                let part1 = (chr2 & 0b00000011) << 6;
                let part2 = chr3 & 0b00111111;
                result.push(part1 + part2);
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod encode {
    use super::*;

    #[test]
    fn should_encode_correctly() -> Result<(), Error> {
        assert_eq!(encode("".as_bytes())?,    "");
        assert_eq!(encode("Man".as_bytes())?, "TWFu");
        assert_eq!(encode("Ma".as_bytes())?,  "TWE=");
        assert_eq!(encode("M".as_bytes())?,   "TQ==");

        Ok(())
    }

}

#[cfg(test)]
mod decode {
    use super::*;

    #[test]
    fn should_decode_full_triplet_correctly() -> Result<(), Error> {
        assert_eq!(decode("TWFu".as_bytes())?, "Man".as_bytes());
        Ok(())
    }

    #[test]
    fn should_decode_two_chars_with_padding_correctly() -> Result<(), Error> {
        assert_eq!(decode("TWE=".as_bytes())?, "Ma".as_bytes());
        Ok(())
    }

    #[test]
    fn should_decode_two_chars_without_padding_correctly() -> Result<(), Error> {
        assert_eq!(decode("TWE".as_bytes())?, "Ma".as_bytes());
        Ok(())
    }

    #[test]
    fn should_decode_one_char_with_padding_correctly() -> Result<(), Error> {
        assert_eq!(decode("TQ==".as_bytes())?, "M".as_bytes());
        Ok(())
    }

    #[test]
    fn should_decode_one_char_without_padding_correctly() -> Result<(), Error> {
        assert_eq!(decode("TQ".as_bytes())?, "M".as_bytes());
        Ok(())
    }

    #[test]
    fn should_decode_concatenated_and_padded_triplets_correctly() -> Result<(), Error> {
        assert_eq!(decode("TWE=TQ==TWFu".as_bytes())?, "MaMMan".as_bytes());
        Ok(())
    }
}
