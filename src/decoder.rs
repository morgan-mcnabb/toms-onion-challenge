use std::fs::File;
use std::io::prelude::*;
use std::char;
use regex::Regex;

pub struct Decoder
{
    offset: u32,
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
            tuple: 0,
            pow85: vec![85*85*85*85, 85*85*85, 85*85, 85, 1],
            decoded_string: String::new() ,
        }
    }

    pub fn begin_decoding_payload(&mut self, encoded_string: &str, payload_num: i32) 
    {   
        let beginning_payload = Regex::new("<~").unwrap().find(&encoded_string).unwrap().start();
        let end_payload = Regex::new("~>").unwrap().find(&encoded_string).unwrap().end();
        let instruction_string = &encoded_string[0..beginning_payload];
        let illegal_char_payload = &encoded_string[beginning_payload + 2..end_payload - 2];
        let mut legal_char_payload = remove_newlines(&illegal_char_payload);
        
        //println!("{}", &encoded_string);
        self.decode_ascii85(&legal_char_payload);
        write_to_file(payload_num, instruction_string, &legal_char_payload);
    }

    pub fn decode_ascii85(&mut self, ascii_string: &str)
    {
        let mut count = 0;
        let ascii85_chars: Vec<char> = ascii_string.chars().collect();
        for (i, c) in ascii85_chars.iter().enumerate()
        {
            self.tuple += ((*c as u32 - self.offset) * self.pow85[count]) as u32;
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
        println!("Im here");
        let characters: Vec<char> = self.decoded_string.chars().collect();
        self.decoded_string.clear();
        for mut c in characters
        {
            c = flip_bits(c);
            self.decoded_string.push(c);
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
        let mut decoded_byte: u8;
        for i in 0..length
        {
            decoded_byte = (self.tuple >> 24 - (i * 8)) as u8;
            self.decoded_string.push(decoded_byte as char);
        }   
        self.tuple = 0;
    }
}


pub fn flip_bits(character: char) -> char
{
    let mask: u8 = 0b0000_0001;
    let mut original_bits = character as u8;
    for _i in 0..4
    {
        original_bits ^= mask;
        original_bits = original_bits.rotate_right(2);
    }
    original_bits = original_bits.rotate_right(1);
    original_bits as char
}


fn write_to_file(payload_num: i32, instructions_string: &str, payload_string: &str) -> std::io::Result<()>
{
    let mut instructions_filename = String::from("instructions_");
    let mut payload_filename = String::from("payload_");

    instructions_filename.push(char::from_digit(payload_num as u32, 10).unwrap());
    payload_filename.push(char::from_digit(payload_num as u32, 10).unwrap());
    instructions_filename.push_str(".txt");
    payload_filename.push_str(".txt");

    let mut instructions_file = File::create(instructions_filename)?;
    let mut payload_file = File::create(payload_filename)?;

    instructions_file.write_all(instructions_string.as_bytes())?;
    payload_file.write_all(payload_string.as_bytes())?;

    Ok(())
}

fn remove_newlines(illegal_string: &str) -> String
{
    str::replace(&illegal_string, "\n", "")
    
}

