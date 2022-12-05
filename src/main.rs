mod cipher;
use colored::Colorize;

fn main() -> std::io::Result<()>
{
    let mut input_mode: String = String::new();

    println!("enter mode ({} / {}):", "shift".cyan(), "smart".magenta());
    std::io::stdin().read_line(&mut input_mode)?;

    if input_mode.trim_end() == "shift"
    {
        // string to shift
        let mut input_string: String = String::new();
        println!("\nenter a string to {}:", "shift".cyan());

        std::io::stdin().read_line(&mut input_string)?;
        input_string = String::from(input_string.trim_end());

        // shift key
        let mut input_key: String = String::new();
        println!("\nenter a value to {} the string by:", "shift".cyan());

        std::io::stdin().read_line(&mut input_key)?;
        let input_key: u8 = input_key.trim().parse().expect("failed to parse integer from input key");

        // result
        println!("\n{}", cipher::shift(&input_string, &input_key).green());
    }
    else if input_mode.trim_end() == "smart"
    {
        // string to intelligently decode
        let mut input_string: String = String::new();
        println!("\nenter a string to {}:", "decode".magenta());

        std::io::stdin().read_line(&mut input_string)?;
        input_string = String::from(input_string.trim_end());

        // result
        let (key, string, confidence) = cipher::smart_decode(&input_string);
        println!
        (
            "\n{} decoded with key {} at {}{} confidence",
            string.green(),
            key.to_string().red(),
            confidence.to_string().yellow(),
            "%".yellow()
        );
    }
    else
    {
        println!("\nplease input a valid option ({} / {}):", "shift".cyan(), "smart".magenta());
    }

    return Ok(());
}
