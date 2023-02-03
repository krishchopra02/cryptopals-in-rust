use std::io;
use std::str;
fn main() {
    let mut hex_string = String::new();
    println!("Enter the hex string: ");
    io::stdin().read_line(&mut hex_string).unwrap();
    println!("{:?}",convert_hex_to_base64(&hex_string));
    
}

fn convert_hex_to_base64(hex_string: &str)->Result<String,&'static str>   {
    let chunked_hex_code: Vec<String> = hex_string.chars().map(|a| a.to_string()).collect();
    let mut bit_string = String::new();
    for i in 0..chunked_hex_code.len()  {
        bit_string += &convert_hex_to_bin(&chunked_hex_code[i]).unwrap();
    }
    let chunked_bit_string = bit_string
        .as_bytes()
        .chunks(6)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    let mut base_64_string = String::new();
    for bits in chunked_bit_string {
        base_64_string += &convert_bin_to_base_64(bits).unwrap();
    }
    
   Ok(base_64_string)
}

fn convert_hex_to_bin(hex: &str) -> Result<String, &str> {
    let result = match hex {
        "0" => "0000",
        "1" => "0001",
        "2" => "0010",
        "3" => "0011",
        "4" => "0100",
        "5" => "0101",
        "6" => "0110",
        "7" => "0111",
        "8" => "1000",
        "9" => "1001",
        "a" => "1010",
        "b" => "1011",
        "c" => "1100",
        "d" => "1101",
        "e" => "1110",
        "f" => "1111",
        "\n" => "1010",
        _ => "Unexpected value",
    };
    if result == "Unexpected value" {
        return Err(result);
    }
    return Ok(result.to_string());
}

fn convert_bin_to_base_64(bin: &str) -> Result<String, &str> {
    let base_64_index_table =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string();
        
    let idx = usize::from_str_radix(bin, 2).unwrap();
    return Ok(base_64_index_table.split("").collect::<Vec<&str>>()[idx + 1].to_string());
}

#[test]
fn simple_test() {
    assert_eq!(convert_hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap(),"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}
