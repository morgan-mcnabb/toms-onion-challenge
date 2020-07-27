mod decoder;

use decoder::Decoder;
use std::env;
use std::fs;
use std::fs::File;
use regex::Regex;

const ONION_LAYERS: i32 = 4;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut payload: String = String::from("");

    let mut decoder: Decoder = decoder::Decoder::new();

    let mut payload_number = 0;


    while payload_number < 3
    {
        if payload_number != 0
        {
            decoder.begin_decoding_payload(&payload, payload_number);
        }

        match payload_number
        {
            0 => {
                payload = fs::read_to_string(filename).expect("Something went wrong reading the file");
                decoder.begin_decoding_payload(&payload, payload_number);
            },
            1 => decoder.decode_second_payload(),
            _ => (),
        } 
        payload.clear();
        payload = decoder.get_decoded_string();
        decoder.clear_decoded_string();
        payload_number += 1;

        
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn string_char_test()
    {
        let string = String::from("a");
        let mut stringtest = String::from("");

        for c in string.chars()
        {
            stringtest.push_str(&c.to_string());
        }

        assert_eq!("a", stringtest);
    }

    #[test]
    fn regex_test()
    {
        let string = "<~sedfgasekfgmaserqwer~>";
        let re = Regex::new("<~(.*)~>").unwrap().find(&string).unwrap();
        assert_eq!(re.start(), 0);
        assert_eq!(re.end(), 36);
    }

   // #[test]
   // fn flip_bits_and_shift_test()
   // {
   //     let mut byte: u8 = 0b1011_0100;
   //     byte = decoder::flip_bits(byte as char) as u8;

   //     assert_eq!(0b1111_0000, byte);

   // }

   // #[test]
   // fn rotate_bits_test()
   // {
   //     let mut byte: u8 = 0b1001_0101;
   //     byte = byte.rotate_right(1);

   //     assert_eq!(0b1100_1010, byte);
   // }

   // #[test]
   // fn counting_set_bits_test()
   // {
   //     let byte: u8 = 158;
   //     let count_of_set_bits = decoder::count_set_bits(byte);

   //     assert_eq!(5, count_of_set_bits);
   // }

   // #[test]
   // fn determining_valid_byte()
   // {
   //     let byte1  = 0xA2;
   //     let byte2 = 0xA3;
   //     let byte3: u8 = 158;
   //     let byte4: u8 = 0b1001_1110;
   //     let count1 = decoder::count_set_bits(byte1);
   //     let count2 = decoder::count_set_bits(byte2);
   //     let count3 = decoder::count_set_bits(byte3);
   //     let count4 = decoder::count_set_bits(byte4);
   //     let is_byte1_valid: bool = decoder::determine_if_byte_is_valid(byte1, count1);
   //     let is_byte2_valid: bool = decoder::determine_if_byte_is_valid(byte2, count2);
   //     let is_byte3_valid: bool = decoder::determine_if_byte_is_valid(byte3, count3);
   //     let is_byte4_valid: bool = decoder::determine_if_byte_is_valid(byte4, count4);

   //     assert!(!is_byte1_valid);
   //     assert!(is_byte2_valid);
   //     assert!(!is_byte3_valid);
   //     assert!(!is_byte4_valid);
   // }

   // #[test]
   // fn space_is_invalid_character_test()
   // {
   //     let byte = ' ' as u8;

   //     let count = decoder::count_set_bits(byte);
   //     let valid = decoder::determine_if_byte_is_valid(byte, count);
   //     assert!(!valid);
   // }
}
