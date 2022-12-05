// shift string by key amount to the right (wrapping at the end of the alphabet)
pub fn shift(string: &String, key: &u8) -> String
{
    let mut new_string: String = String::with_capacity(string.len());
    for character in string.chars()
    {
        if character == ' '
        {
            new_string.push(' ');
            continue;
        }

        if character >= 'a' && character <= 'z'
        {
            new_string.push(((((character as u8 + key) - 'a' as u8) % 26) + 'a' as u8) as char);
        }
        else if character >= 'A' && character <= 'Z'
        {
            new_string.push(((((character as u8 + key) - 'A' as u8) % 26) + 'A' as u8) as char);
        }
        else
        {
            println!("{} != {}", character as u8, 'a' as u8);
            panic!("non ascii string entered.");
        }
    }

    return new_string;
}

// given an encoded string, returns the shift key, decoded string, and confidence value in percent
// if it does not find any valid value, will default to 0 and the default string
pub fn smart_decode(string: &String) -> (u8, String, f32)
{
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::collections::HashMap;

    let wordlist = match File::open(".\\wordlist.txt")
    {
        Ok(list) => list,
        Err(_) => panic!("couldnt open wordlist in smart_decode")
    };

    let mut len_without_spaces: usize = string.len();
    for character in string.chars()
    {
        if character == ' '
        {
            len_without_spaces -= 1;
        }
    }

    let mut key_map: HashMap<u8, u32> = HashMap::new();

    for line in io::BufReader::new(&wordlist).lines()
    {
        if let Ok(line) = line
        {
            for key in 0..26
            {
                for word in string.split(' ')
                {
                    if self::shift(&word.to_string(), &key).to_ascii_lowercase() == line
                    {
                        *(key_map.entry(key).or_insert(0) as &mut u32) += word.len() as u32;
                    }
                }
            }
        }
        else
        {
            println!("failed to get current line as string")
        }
    }

    let mut largest_key: u8 = 0;
    let mut largest_value: &u32 = &0;
    for key in 1..26
    {
        if let Some(current_entry) = key_map.get_key_value(&key)
        {
            if current_entry.1 > largest_value
            {
                largest_key = key;
                largest_value = current_entry.1;
            }
            else
            {
                continue;
            }
        }
        else
        {
            continue;
        }
    }

    let largest_string: String = self::shift(&string, &largest_key);
    let confidence: f32 = (key_map[&largest_key] as f32 / len_without_spaces as f32) * 100.0;

    return (largest_key, largest_string, confidence);
}
