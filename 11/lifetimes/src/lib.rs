// Need to be explicit in saying that a and b will last the same length as each other
// and that the return value exists while they exist
pub fn greatest<'a>(a: &'a str, b: &'a str) -> Option<&'a str> {
    if a.len() == b.len() {
        return None;
    }
    if b.len() > a.len() {
        return Some(b);
    } else {
        return Some(a);
    }
}

// Specify that the references have the same lifetime
pub struct Config<'a> {
    pub first: &'a str,
    pub second: &'a str,
}

pub fn run(cfg: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let hello = "Hello".to_string();
    let _world = "world".to_string();

    let result;
    {
        let combined = "sh".to_string();
        // This is not ok because the function greatest expects both to have the same life time
        // the reality is that the 'combined' doesn't live as long as 'hello' so there's a chance for a dangling pointer
        //result = greatest(&hello, &combined);

        // This does compile because the return goes out of scope at the sametime/before the variables.
        // So no chance of dangling pointer
        let _res = greatest(&combined, &hello).unwrap_or("Same length");

        // Even here I need to make a new string from the reference so that the reference isn't dangling
        result = String::from(_res);
    }
    println!("Static Greatest of '{}' and '{}' is '{}'", "out of scope \"sh\"", hello, result);

    println!(
        "Greatest of '{}' and '{}' is '{}'",
        cfg.first,
        cfg.second,
        greatest(&cfg.first, &cfg.second).unwrap_or("neither")
    );
    Ok(())
}

pub fn parse_args(args: &[String]) -> Result<Config, &'static str> {
    if args.len() != 2 {
        return Err("Not enough args");
    }

    Ok(Config {
        first: &args[0],
        second: &args[1],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_left() {
        let l = "one";
        let r = "second";
        let result = greatest(l, r).unwrap_or("failure");
        assert_eq!(result, "second");
    }

    #[test]
    fn longest_right() {
        let l = "one";
        let r = "seconds";
        let result = greatest(r, l).unwrap_or("failure");
        assert_eq!("seconds", result);
    }

    #[test]
    fn equal() {
        let l = "one";
        let r = "two";
        let result = greatest(l, r);
        assert_eq!(None, result);
    }
}
