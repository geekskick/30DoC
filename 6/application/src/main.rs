use std::ops::Range;
// Not sure about this yet
fn show_range<T: std::ops::Sub + std::fmt::Display>(r: &Range<T>){
    println!("{}..{} is {}",r.start, r.end, r.end - r.start);
}
fn main() {
    println!("Hello, world I am counting using a for loop!");
    for number in 1..4{
        println!("{}", number);
    }

    let things = ["Hello", "you","cunt"];
    for thing in things.iter(){
        println!("Thing = {}", thing);
    }

    let mut counter = 0; // I can't imagine that this is good practice
    let result = loop{
        counter += 1;
        if counter == 12 {
            break counter;
        }
    };

    println!("Result = {}", result);

    for thing in things.iter().rev(){
        println!("Backwards thing = {}", thing);
    }

    let sentence = "Hello you cunting sentence";
    for word in sentence.split_ascii_whitespace(){
        println!("Word = {}", word);
    }

    for (idx, word) in sentence.split_ascii_whitespace().enumerate(){
        println!("Word {} = {}",idx,word);
    }

    for (word, thing) in sentence.split_ascii_whitespace().zip(things.iter()){
        println!("Iter = ({},{})",word,thing);
    }

    let term_sentence = "this_doesn't_have_whitespace";
    for word in term_sentence.split_terminator("_"){
        println!("word = {}", word);
    }

    let r = 0..4;
    show_range(&r);

    //let r = 4..0; this fails cause it's a usize and 0-4 is an underflow
    // How to get the number of elements in a range?
    //show_range(&r);

}
