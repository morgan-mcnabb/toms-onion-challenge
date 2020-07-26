mod decoder;

use decoder::Decoder;
use std::env;
use std::fs;

const ONION_LAYERS: i32 = 4;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    // let illegal_char_string = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // let first_encoded_string = remove_newline(&illegal_char_string);
    // let mut decoder = Decoder::new();
    // decoder.decode_ascii85(first_encoded_string);
    // 
    // let first_decoded_string = decoder.get_decoded_string();
    // decoder.clear_decoded_string();
    //// println!("{}", &first_decoded_string);
    // 

    // let start_bytes = first_decoded_string.find("<~").unwrap();
    // let end_bytes = first_decoded_string.find("~>").unwrap();

    // let illegal_char_string = &first_decoded_string[start_bytes+2..end_bytes];
    // let second_encoded_string = remove_newline(&illegal_char_string);
    // 
    // let second_decoded_string = decoder.decode_ascii85(second_encoded_string);

    // decoder.decode_second_payload();

    // let second_string_with_rules_applied = decoder.get_decoded_string();
    // decoder.clear_decoded_string();
    //// println!("{}", second_string_with_rules_applied);
    // 
    // let start_bytes = second_string_with_rules_applied.find("<~").unwrap();
    // let end_bytes = second_string_with_rules_applied.find("~>").unwrap();
    // 
    let mut payload = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut decoder: Decoder = decoder::Decoder::new();
    
    let mut payload_number = 0;

    while payload_number < 0
    {
        decoder.begin_decoding_payload(payload, payload_number);
        match payload_number
        {
            _ => (),
            //1 => decoder.decode_second_payload(),
         //   2 => decoder.decode_third_payload(),
        }
        payload = decoder.get_decoded_string();
        decoder.clear_decoded_string();
        payload_number += 1;
    }
    

    decoder.begin_decoding_payload(payload, 0);

    let second_payload_encoded = decoder.get_decoded_string();
    decoder.clear_decoded_string();

    decoder.begin_decoding_payload(second_payload_encoded, 1);
    decoder.decode_second_payload();

    let third_payload_encoded = decoder.get_decoded_string();
    decoder.clear_decoded_string();
    println!("{}", third_payload_encoded);
//    decoder.begin_decoding(third_payload_encoded);
//    decoder.decode_third_payload();

//    let fourth_payload_encoded = decoder.get_decoded_string();
//    println!("{}", fourth_payload_encoded);

}
//fn begin_decoding(decoder: &Decoder, encoded_string: String) 
//{
//    let start_bytes = encoded_string.find("<~").unwrap_or(0);
//    let end_bytes = encoded_string.find("~>").unwrap_or(encoded_string.len());
//    let illegal_char_string = encoded_string[start_bytes + 2..end_bytes];
//    let legal_char_string = remove_newlines(illegal_char_string);
//
//    decoder.decode_ascii85(legal_char_string);
//}
//



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
    fn flip_bits_and_shift_test()
    {
        let mut byte: u8 = 0b1011_0100;
        byte = decoder::flip_bits(byte as char) as u8;

        assert_eq!(0b1111_0000, byte);

    }

    #[test]
    fn rotate_bits_test()
    {
        let mut byte: u8 = 0b1001_0101;
        byte = byte.rotate_right(1);

        assert_eq!(0b1100_1010, byte);
    }

    #[test]
    fn counting_set_bits_test()
    {
        let byte: u8 = 15;
        let count_of_set_bits = decoder::count_set_bits(0xA2);

        assert_eq!(3, count_of_set_bits);
    }

    #[test]
    fn determining_valid_byte()
    {
        let byte1  = 0xA2;
        let byte2 = 0xA3;
        let count1 = decoder::count_set_bits(byte1);
        let count2 = decoder::count_set_bits(byte2);
        let is_byte1_valid: bool = decoder::determine_if_byte_is_valid(byte1, count1);
        let is_byte2_valid: bool = decoder::determine_if_byte_is_valid(byte2, count2);

        assert!(!is_byte1_valid);
        assert!(is_byte2_valid);
    }
}
