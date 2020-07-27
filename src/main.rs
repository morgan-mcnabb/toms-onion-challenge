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
}
