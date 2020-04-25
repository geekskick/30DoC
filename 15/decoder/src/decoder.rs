use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Direction {
    To,
    From,
}

// Code Review comment: There are traits which define converting to/from strings and stuff.
// Using these traits makes the code more rusty
impl std::str::FromStr for Direction{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "to" => Ok(Direction::To),
            "from" => Ok(Direction::From),
            _ => Err("Direction arg must be 'to' or 'from'"),
        }
    }
}

pub struct Config {
    pub direction: Direction,
    pub message: String,
}

pub fn parse_args(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("Not enough args");
    }

    // Code review comment: able to use ? to return the error from the function
    // if it's returned
    let dir = Direction::from_str(&args[1])?;

    let msg = args[2..].join(" ");
    Ok(Config {
        direction: dir,
        message: msg,
    })
}

pub fn run(cfg: &Config) -> Result<(), &'static str> {
    match cfg.direction {
        Direction::To => match encode(&cfg.message) {
            None => return Err("Unable to encode"),
            Some(s) => println!("{}", s),
        },
        Direction::From => match decode(&cfg.message) {
            None => return Err("Unable to decode"),
            Some(s) => println!("{}", s),
        },
    }
    Ok(())
}

fn decode(morse: &str) -> Option<String> {
    // In code review someone pointed out that using .to_string() makes a string object,
    // which uses heap allocation so it's of course not a great idea. Changing this to str
    // means it's on the stack. In order to use str you have to be explicit about it's lifetime
    // for some reason?!
    // * String - https://doc.rust-lang.org/std/string/struct.String.html
    // * str    - https://doc.rust-lang.org/std/primitive.str.html
    let map: std::collections::HashMap<&'static str, &'static str> = [
        (".-", "a"),
        ("-...", "b"),
        ("-.-.", "c"),
        ("-..", "d"),
        (".", "e"),
        ("..-.", "f"),
        ("--.", "g"),
        ("....", "h"),
        ("..", "i"),
        (".---", "j"),
        ("-.-", "k"),
        (".-..", "l"),
        ("--", "m"),
        ("-.", "n"),
        ("---", "o"),
        (".--.", "p"),
        ("--.-", "q"),
        (".-.", "r"),
        ("...", "s"),
        ("-", "t"),
        ("..-", "u"),
        ("...-", "v"),
        (".--", "w"),
        ("-..-", "x"),
        ("-.--", "y"),
        ("--..", "z"),
        ("/", " "),
        ("-----", "0"),
        (".----", "1"),
        ("..---", "2"),
        ("...--", "3"),
        ("....-", "4"),
        (".....", "5"),
        ("-....", "6"),
        ("--...", "7"),
        ("---..", "8"),
        ("----.", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    // Code review comment: Someone pointed out that it's better to use ::new()
    // rather than "".to_string(), not sure why yet though
    let mut rc = String::new();
    for word in morse.split_ascii_whitespace() {
        rc += map.get(word)?;
    }
    Some(rc)
}

fn encode(msg: &str) -> Option<String> {
    // Code review comment: Using vec is dynamic when the lenghts are known at compile time this isn't the best.
    // I suppose it's similar to std::vector and std::array

    // In addition these need to be const - this is so thet allow for greater compile time optimisation,
    // let is evaluated at runtume and consts are basically static
    const TABLE : [&str; 26] = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--.."
    ];
    const NUMBER_TABLE : [&str; 10] = [
        "-----", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----."
    ];

    let mut rs = String::new();

    for (idx, letter) in msg.chars().enumerate() {
        if letter.is_numeric() {
            // Code review comment said I can use the ? operator
            // This tries to get the Some() out, if not it returns None
            let number = letter.to_digit(10)?;
            rs += NUMBER_TABLE[number as usize];
        } else if letter.is_ascii_alphabetic() {
            let letter = letter.to_ascii_lowercase();
            let number = letter as u8;
            // If I want a byte I need to use b'a'. 'a' is a char type which is 4 bytes long
            // Thanks cargo clippy 
            let number = number - b'a';
            rs += TABLE[number as usize];
        } else if letter.is_ascii_whitespace() {
            rs += "/";
        } else {
            return None;
        }

        if idx != msg.len() - 1 {
            rs += " ";
        }
    }

    Some(rs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test() {
        let message = "hello";
        let morse = encode(&message);
        assert_eq!(morse.unwrap(), ".... . .-.. .-.. ---");

        let message = "1";
        let morse = encode(&message);
        assert_eq!(morse.unwrap(), ".----");

        let message = "hello there";
        let morse = encode(&message);
        assert_eq!(morse.unwrap(), ".... . .-.. .-.. --- / - .... . .-. .");
    }

    #[test]
    fn decode_test() {
        let message = ".... . .-.. .-.. ---";
        let human = decode(&message);
        assert_eq!("hello", human.unwrap());

        let message = ".... . .-.. .-.. --- / - .... . .-. .";
        let human = decode(&message);
        assert_eq!("hello there", human.unwrap());

        let message = ".----";
        let human = decode(&message);
        assert_eq!(human.unwrap(), "1");
    }

    #[test]
    fn direction_parse_test() {
        assert!(Direction::from_str("message").is_err());
        assert_eq!(Direction::From, Direction::from_str("from").unwrap());
        assert_eq!(Direction::To, Direction::from_str("to").unwrap());
    }

    #[test]
    fn parse_args_test() {
        let args = [
            "prog_name".to_string(),
            "from".to_string(),
            "message".to_string(),
        ];
        let cfg = parse_args(&args);
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.direction, Direction::From);
        assert_eq!(cfg.message, "message");

        let args = vec!["Not long enough".to_string()];
        let cfg = parse_args(&args);
        assert!(cfg.is_err());

        let args = vec!["some name".to_string(), "some thing else".to_string()];
        let cfg = parse_args(&args);
        assert!(cfg.is_err());

        let args = vec![
            "some name".to_string(),
            "to".to_string(),
            "dir".to_string(),
            "Extra".to_string(),
        ];
        let cfg = parse_args(&args);
        assert_eq!(cfg.unwrap().message, "dir Extra");

        let args = vec![
            "prog_name".to_string(),
            "to".to_string(),
            "message".to_string(),
        ];
        let cfg = parse_args(&args);
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.direction, Direction::To);
        assert_eq!(cfg.message, "message");
    }

}
