
// Need to be explicit in saying that a and b will last the same length as each other
// and that the return value exists while they exist
fn greatest<'a>(a : &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() {
        b
    }else{
        a
    }
}

fn main() {
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
        let _res = greatest(&combined, &hello);

        // Even here I need to make a new string from the reference so that the reference isn't dangling
        result = String::from(_res);
    }

    println!("Greatest = {}", result);
}
