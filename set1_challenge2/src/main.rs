use std::io;
use std::str;
fn main() {
    let mut hex_string1 = String::new();
    println!("Enter the first hex string: ");
    io::stdin().read_line(&mut hex_string1).unwrap();
    let mut hex_string2 = String::new();
    println!("Enter the second hex string: ");
    io::stdin().read_line(&mut hex_string2).unwrap();
    println!("{:?}",fixed_xor(&hex_string1, &hex_string2));

}

fn fixed_xor(hex_str1:&str,hex_str2:&str) -> Result<String,&'static str> {
    let mut bit_string1 = String::new();
    let chunked_hex_string = hex_str1.as_bytes().chunks(1).map(str::from_utf8).collect::<Result<Vec<&str>,_>>().unwrap();
    for char in chunked_hex_string {
        bit_string1+=&convert_hex_to_bin(char).unwrap();
    }
   
    let mut bit_string2 = String::new();
    let chunked_hex_string = hex_str2.as_bytes().chunks(1).map(str::from_utf8).collect::<Result<Vec<&str>,_>>().unwrap();
    for char in chunked_hex_string{
        bit_string2+=&convert_hex_to_bin(char).unwrap();
    }
    let xored_bit_string = xor_bits(&bit_string1,&bit_string2).unwrap();
    let chunked_bit_str = xored_bit_string.as_bytes().chunks(4).map(str::from_utf8).collect::<Result<Vec<&str>,_>>().unwrap();
    let mut xored_string = String::new();
    for bytes in chunked_bit_str {
    xored_string+= &bin_to_hex(bytes).unwrap();
    }
    Ok(xored_string)
}
fn xor_bits(bit1: &str,bit2:&str) -> Result<String,&'static str> {
    if bit1.len() != bit2.len() {
        return Err("Not equal length buffers");
    }
    let mut xored_bit_string = String::new();
    for index in 0..bit1.len() {
        match bit1.chars().collect::<Vec<char>>()[index] {
            '1' => match bit2.chars().collect::<Vec<char>>()[index] {
                '1' => xored_bit_string+="0",
                '0' => xored_bit_string+="1",
                _ => return Err("Invalid value"),
            },
            '0' => match bit2.chars().collect::<Vec<char>>()[index] {
                '1' => xored_bit_string+="1",
                '0' => xored_bit_string+="0",
                _ => return Err("Invalid value"),
            },
            _ => return Err("Invalid value"),
        };
    }

    Ok(xored_bit_string)
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

fn bin_to_hex(bin_str: &str) -> Result<String,&'static str> {
    if bin_str.len() != 4 {
        println!("{}",bin_str.len());
        return Err("Unexpected bit string length");
    }

    let hex_table = "0123456789abcdef";
    let idx = usize::from_str_radix(bin_str, 2).unwrap();
    let hex_val = hex_table.split("").collect::<Vec<&str>>()[idx + 1].to_string();
    return Ok(hex_val);
}

#[test] 
fn simple_test() {
    assert_eq!(fixed_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965").unwrap(),"746865206b696420646f6e277420706c6179");
}