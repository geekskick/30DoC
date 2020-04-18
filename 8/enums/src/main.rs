enum IpAddressKind{
    V4(String), //The V4 enumeration has a string associated with it, this can be any nmber of args and datatypes. If it's loads it's beneficial to use a struct
    V6(String),
}

/*
Syntax above is the same as
struct IpAddress{
    kind: IpAddresKind,
    address: String,
}
*/

#[derive(Debug)]
struct RGB{
    r: u8,
    g: u8,
    b: u8
}

#[derive(Debug)]
enum Message {
    Quit,
    Move{ x : isize, y: isize }, // anonymous struct
    Write(String),
    ChangeColour(RGB), // named struct
}

// enums can have functions too, since an enum in rust is sort of a struct with some additionan "real" enum field
// this makes sense
impl Message{
    fn process(&self){
        println!("I am being processed: {:?}", self);
    }
}

#[derive(Debug)]
enum MyOption<T>{
    Some(T),
    None(String),
}

impl<T> MyOption<T>{
    fn underlying(self)->T{
        // This is a match expression
        match self{
            MyOption::Some(value) => value,
            MyOption::None(err_msg) => panic!("Nothing to get: {}", err_msg) 
        }
    }

    fn underlying_ref(&self) -> &T{
        match self{
            MyOption::Some(value) => value,
            MyOption::None(err_msg) => panic!("No ref to get: {}", err_msg)
        }
    }
}

fn maybe_does_it<T>(info : T, enabled: bool) -> MyOption<T>{
    if enabled {
        MyOption::Some(info)
    }
    else{ 
        MyOption::None("This function wasn't successful".to_string())
    }
}

fn main() {
    println!("Hello,enums");

    let _ = IpAddressKind::V4("127.0.0.1".to_string());
    let _ = IpAddressKind::V6("::1".to_string());

    let m = Message::Write("Some message".to_string());
    m.process();
    let m = Message::ChangeColour(RGB{r:0,g:0,b:0});
    m.process();
    let m = Message::Move{x: 4, y:3};
    m.process();
    let m = Message::Quit;
    m.process();

    let success = maybe_does_it("Hello", true);
    let failure = maybe_does_it("world", false);

    println!("Success = {:?}", success);
    println!("Failure = {:?}", failure);

    let success_value = success.underlying_ref();
    println!("Success value by ref = {}", success_value);
    println!("Success value = {}", success.underlying());

    // Doesn't compile because success has been borrowed from and has been moved out of scope
    // println!("Success value by ref after it's been moved = {}", success_value);

    // Doesn't compile because the call to underlying() moves the value out of scope
    // let success_value = success.underlying();

    // This panics cause theres nothing in the underlying option
    println!("Failure value = {}", failure.underlying());
}
