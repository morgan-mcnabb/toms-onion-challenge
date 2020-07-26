use std::fs::File;
use std::io::prelude::*;
use std::char;

pub struct Decoder
{
    offset: u32,
    encoded_block: Vec<u8>,
    decoded_block: Vec<u8>,
    tuple: u32,
    pow85: Vec<u32>,
    decoded_string: String,
}

impl Decoder
{
    pub fn new() -> Decoder
    {
        Decoder
        {
            offset: 33,
            encoded_block: Vec::new(),
            decoded_block: Vec::new(),
            tuple: 0,
            pow85: vec![85*85*85*85, 85*85*85, 85*85, 85, 1],
            decoded_string: String::new() ,
        }
    }

    pub fn begin_decoding_payload(&mut self, encoded_string: String, payload_num: i32) -> std::io::Result<()> 
    {
        let mut instructions_filename = String::from("instructions_");
        let mut payload_filename = String::from("payload_");
        
        instructions_filename.push(char::from_digit(payload_num as u32, 10).unwrap());
        payload_filename.push(char::from_digit(payload_num as u32, 10).unwrap());
        instructions_filename.push_str(".txt");
        payload_filename.push_str(".txt");




        let mut start_bytes = encoded_string.find("<~").unwrap_or(0);
        let end_bytes = encoded_string.find("~>").unwrap_or(encoded_string.len());
        let illegal_char_string = &encoded_string[start_bytes + 2..end_bytes];
        let legal_char_string = remove_newlines(&illegal_char_string);
        self.decode_ascii85(legal_char_string);
        Ok(())
    }

    pub fn decode_ascii85(&mut self, ascii_string: String)
    {
        let mut count = 0;
        let ascii85_chars: Vec<char> = ascii_string.chars().collect();
        for c in ascii85_chars
        {
            self.tuple += (c as u32 - self.offset) * self.pow85[count];
            count += 1;
            if count == 5
            {
                self.decode_block(4);
                count = 0;
            }
        }

        if count != 0
        {
            count-= 1;
            self.tuple += self.pow85[count];
            self.decode_block(count);

        }
    }

    pub fn decode_second_payload(&mut self)
    {
        let characters: Vec<char> = self.decoded_string.chars().collect();
        self.decoded_string.clear();
        for mut c in characters
        {
            c = flip_bits(c);
            self.decoded_string.push(c);
        }
    }

    pub fn decode_third_payload(&mut self)
    {
        let characters: Vec<char> = self.decoded_string.chars().collect();
        self.decoded_string.clear();
        let mut count = 0;
        
        for mut c in characters
        {
            count = count_set_bits(c as u8);
            if determine_if_byte_is_valid(c as u8, count)
            {
                c = (c as u8 & 0b1111_1110) as char;

                self.decoded_string.push(c);
            }
        }
    }

    pub fn get_decoded_string(&self) -> String
    {
        String::from(&self.decoded_string)
    }

    pub fn clear_decoded_string(&mut self)
    {
        self.decoded_string.clear();
    }


    pub fn decode_block(&mut self, length: usize)
    {
        for i in 0..length
        {
            self.decoded_block.push((self.tuple >> 24 - (i * 8)) as u8);
            self.decoded_string.push(self.decoded_block[i] as char);
        }   
        self.tuple = 0;
        self.decoded_block.clear();
    }
}


pub fn flip_bits(character: char) -> char
{
    let mut mask: u8 = 0b0000_0001;
    let mut original_bits = character as u8;
    for i in (0..4)
    {
        original_bits ^= mask;
        original_bits = original_bits.rotate_right(2);
    }
    original_bits = original_bits.rotate_right(1);
    original_bits as char
}

pub fn count_set_bits(byte: u8) -> u8
{
    let mut count: u8 = 0;
    let mut byte_to_change = byte >> 1;
    for i in 0..7
    {
        count += byte_to_change & 1;
        byte_to_change >>= 1;
    }
    count
}

pub fn determine_if_byte_is_valid(byte: u8, count: u8) -> bool
{
    let parity_bit = byte & 0b0000_0001;

    if count % 2 == 0 
    {
        parity_bit == 0
    }
    else
    {
        parity_bit == 1
    }
}


fn remove_newlines(illegal_string: &str) -> String
{
    str::replace(&illegal_string, "\n", "")
}
