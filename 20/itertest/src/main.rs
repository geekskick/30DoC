use std::thread;
use std::time::Duration;
extern crate rand;
use crate::rand::Rng;

struct Cacher<U: std::fmt::Display + std::hash::Hash + Copy + Eq + PartialOrd, T: Fn(U) -> U> {
    calculate: T,
    map: std::collections::HashMap<U, U>,
}

impl<T: Fn(U) -> U, U: std::fmt::Display + std::hash::Hash + Eq + Copy + PartialOrd> Cacher<U, T> {
    fn new(calculation: T) -> Cacher<U, T> {
        Cacher {
            calculate: calculation,
            map: std::collections::HashMap::new(),
        }
    }

    fn value(&mut self, number: U) -> U {
        if self.map.contains_key(&number){
            return self.map.get(&number).unwrap().clone();
        }
        let v = (self.calculate)(number);
        self.map.insert(number, v);
        v
    }
}

fn generate_workout<U: std::convert::From<u32> + std::fmt::Display + std::hash::Hash + Copy + Eq + PartialOrd, T: Fn(U) -> U>(intensity: U, random: U, cache : &mut Cacher<U, T>) {

    if intensity < U::from(25) {
        println!("Do {} pushups", cache.value(random));
        println!("Do {} situps", cache.value(random));
    } else if random == U::from(3) {
        println!("Take a break");
    } else {
        println!("Do {} running", cache.value(random));
    }
}

/*
struct Mover<U: std::fmt::Display, F : FnOnce(U) -> U>{
    dewit : F,
    val : U
}

impl<U: std::fmt::Display, F: FnOnce(U)->U> Mover<U, F>{
    fn get(&self, v : U) -> U{
        let result = (self.dewit)(v);
        // can't use v because FnOnce moves it away
        println!("dewit on {} = {}", v, result);
        result
    }
}
*/

struct Muter<U: std::default::Default + std::fmt::Display, F: FnMut(&mut U) -> U>{
    dewit : F,
    val : U
}
impl<U: std::default::Default + std::fmt::Display, F: FnMut(&mut U) -> U>  Muter<U, F>{
    fn get(&mut self, mut v : U) -> U{
        let result = (self.dewit)(&mut v);
        println!("fn on {} = {}", v, result);
        result
    }
    fn new(d : F) -> Muter<U, F>{
        Muter{
            dewit : d,
            val : U::default()
        }
    }
}

#[derive(Debug)]
struct Shoe{
    make : &'static str,
    size: u64
}

impl Shoe{
    fn new(make : &'static str) -> Shoe{
        let mut rng = rand::thread_rng();
        Shoe{
            make : make,
            size : rng.gen_range(0, 20)
        }
    }
}

impl std::fmt::Display for Shoe{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Make {}, Size {}", self.make, self.size)
    }
}

fn main() {

    let mut result = Cacher::new(|num: u32| {
        println!("Doing closure: {}", num);
        thread::sleep(Duration::from_secs(2));
        num
    });
    let mut user_value : u32 = 10;
    let random_number : u32 = 8;
    generate_workout(user_value, random_number, &mut result);
    generate_workout(user_value, random_number * 2, &mut result);
    generate_workout(user_value, random_number, &mut result);

    let mut m = Muter::new(|num : &mut u32|{
        let r = *num;
        *num = 0;
        r
    });

    println!("m = {}", m.get(user_value));

    let shoes = vec![
        Shoe::new("Nike"), Shoe::new("Adidas"), Shoe::new("Ford"), Shoe::new("make: &'static str")
    ];

    let my_shoe_sizes : Vec<u64> = shoes.iter().map(|s| s.size).collect();
    println!("Shoes : {:?}", shoes);
    println!("Shoe sizes : {:?}", my_shoe_sizes);
    let shoes_what_fit : Vec<&Shoe> = shoes.iter().filter(|s| s.size <= 11).collect();
    println!("Shoes that'll fit me: {:?}", shoes_what_fit);
}
