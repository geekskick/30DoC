
#[derive(Debug,PartialEq)]
pub enum Direction{
    To, 
    From,
}

impl Direction{
    pub fn from(s : &str) -> Option<Direction>{
        match s {
            "to" => Some(Direction::To),
            "from" => Some(Direction::From),
            _ => None
        }
    }
}


pub struct Config{
    pub direction : Direction,
    pub message: String,
}

pub fn parse_args(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("Not enough args");
    }

    let dir = Direction::from(&args[1]);
    if dir.is_none(){
        return Err("Must choose either 'from' or 'to' as direction");
    }

    let msg = args[2..].join(" ");
    Ok(Config{direction: dir.unwrap(), message: msg})
}

pub fn run(cfg : &Config) -> Result<(), &'static str>{
    match cfg.direction {
        Direction::To =>{
            match encode(&cfg.message) {
                None => return Err("Unable to encode"),
                Some(s) => println!("{}", s),
            }
        },
        Direction::From => {
            match decode(&cfg.message){
                None => return Err("Unable to decode"),
                Some(s) => println!("{}", s),
            }
        }
    }
    Ok(())
}

fn decode(morse : &str) -> Option<String>{
    let map : std::collections::HashMap<String, String> = [
        (".-".to_string(), "a".to_string()),
        ("-...".to_string(), "b".to_string()),
        ("-.-.".to_string(), "c".to_string()),
        ("-..".to_string(), "d".to_string()),
        (".".to_string(), "e".to_string()),
        ("..-.".to_string(), "f".to_string()),
        ("--.".to_string(), "g".to_string()),
        ("....".to_string(), "h".to_string()),
        ("..".to_string(), "i".to_string()),
        (".---".to_string(), "j".to_string()),
        ("-.-".to_string(), "k".to_string()),
        (".-..".to_string(), "l".to_string()),
        ("--".to_string(), "m".to_string()),
        ("-.".to_string(), "n".to_string()),
        ("---".to_string(), "o".to_string()),
        (".--.".to_string(), "p".to_string()),
        ("--.-".to_string(), "q".to_string()),
        (".-.".to_string(), "r".to_string()),
        ("...".to_string(), "s".to_string()),
        ("-".to_string(), "t".to_string()),
        ("..-".to_string(), "u".to_string()),
        ("...-".to_string(), "v".to_string()),
        (".--".to_string(), "w".to_string()),
        ("-..-".to_string(), "x".to_string()),
        ("-.--".to_string(), "y".to_string()),
        ("--..".to_string(), "z".to_string()),
        ("/".to_string(), " ".to_string()),
        ("-----".to_string(), "0".to_string()),
        (".----".to_string(),"1".to_string()),
        ("..---".to_string(),"2".to_string()),
        ("...--".to_string(), "3".to_string()),
        ("....-".to_string(),"4".to_string()),
        (".....".to_string(), "5".to_string()),
        ("-....".to_string(), "6".to_string()),
        ("--...".to_string(), "7".to_string()),
        ("---..".to_string(),"8".to_string()),
        ("----.".to_string(),"9".to_string())
    ].iter().cloned().collect();

    let mut rc = "".to_string();
    for word in morse.split_ascii_whitespace(){
        match map.get(word){
            None => return None,
            Some(s) => rc += s,
        }
    }
    Some(rc)
}

fn encode(msg : &str) -> Option<String>{
    let table = vec![".-", "-...", "-.-.", "-..", ".", "..-.","--.","....", "..", ".---","-.-", ".-..", "--","-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."];
    let number_table = vec!["-----", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----.", "-----"];
    let mut rs = "".to_string();

    for letter in msg.chars(){
        if letter.is_numeric(){
            let number = letter.to_digit(10);
            match number{
                None => return None,
                Some(n) => rs += number_table[n as usize],
            }
        }
        else if letter.is_ascii_alphabetic(){
            let letter = letter.to_ascii_lowercase();
            let number = letter as u8;
            let number = number - ('a' as u8);
            rs += table[number as usize];
        }
        else if letter.is_ascii_whitespace(){
            rs += "/";
        }
        rs += " ";
    }

    Some(String::from(rs.trim_end()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test(){
        let message = "hello".to_string();
        let morse = encode(&message);
        assert_eq!(morse.unwrap(), ".... . .-.. .-.. ---");

        let message = "1".to_string();
        let morse = encode(&message);
        assert_eq!(morse.unwrap(), ".----");

        let message = "hello there";
        let morse = encode(&message);
        assert_eq!(morse.unwrap(), ".... . .-.. .-.. --- / - .... . .-. .");
    }

    #[test]
    fn decode_test(){
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
        assert_eq!(None, Direction::from("message"));
        assert_eq!(Some(Direction::From), Direction::from("from"));
        assert_eq!(Some(Direction::To), Direction::from("to"));
    }

    #[test]
    fn parse_args_test(){
        let args = vec!["prog_name".to_string(), "from".to_string(), "message".to_string()];
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

        let args = vec!["some name".to_string(), "to".to_string(), "dir".to_string(), "Extra".to_string()];
        let cfg = parse_args(&args);
        assert_eq!(cfg.unwrap().message, "dir Extra");

       let args = vec!["prog_name".to_string(), "to".to_string(), "message".to_string()];
        let cfg = parse_args(&args);
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.direction, Direction::To);
        assert_eq!(cfg.message, "message"); 
    }


}