// set key as positive to "encode" ana negative to "decode"
pub fn shift(string: &String, key: i16) -> String
{
    let mut new_string: String = String::with_capacity(string.len());
    for character in string.chars()
    {
        new_string.push((character as i16 + (key % 26)) as u8 as char);
    }

    return new_string;
}

pub fn smart_decode(string: &String) -> String
{
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::collections::HashMap;

    let wordlist = match File::open(".\\wordlist.txt")
    {
        Ok(list) => list,
        Err(_) => panic!("couldnt open wordlist in smart_decode")
    };

    let key_map: HashMap::<usize, u32> = HashMap::new();

    for word in string.split(' ')
    {
        for line in io::BufReader::new(wordlist).lines()
        {

        }
    }
    return String::new();
}
