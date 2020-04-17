
fn find_max(numbers: &[i32])->i32{
    let mut max = numbers[0];
    for &number in numbers{
        if number > max{
            max = number
        }
    }
    max
}

fn generic_ref_max<T:PartialOrd>(numbers: &[T]) -> &T{
    let mut max = &numbers[0];
    for number in numbers{
        if number > max{
            max = number;
        }
    }
    max
}

fn generic_max<T: PartialOrd + Copy>(numbers: &[T])->T{
    let mut max = numbers[0];
    for &number in numbers{
        if number > max{
            max = number
        }
    }
    max
}

struct Point<T,U>{
    x : T,
    y : U
}

// Only provide an implementation for structs where T and U are f32s
// template specialization
impl Point<f32, f32>{
    fn distance(&self)->f32{
        // Who cares
        self.x.sin() + self.y.cos()
    }
}

impl<T, U> Point<T, U>{
    // Can have the function itself have additional generics if needed
    // In this case these are different to the  struct's types
    fn swap<W, V>(self, other :Point<W, V>) -> Point<T, V>{
        Point{x : self.x, y: other.y }
    }
}

// Like an abstract interface
trait Summary{
    fn summarise(&self)->String;
    fn defaulted_fn(&self)->String{
        "Defaulted".to_string()
    }

    fn something_else(&self)->String{
        format!("Im a type: {}", self.summarise())
    }
}

impl<T, U> Summary for Point<T, U>{
    fn summarise(&self)->String{
        "I'm a point!".to_string()
    }
    fn defaulted_fn(&self)->String{
        "I've overriden the defaulted behaviour".to_string()
    }

}

// The implementation of the summary trait for the type i32 is possible
// because the trait is local to this module(?) even though i32 is a stdlib type
impl Summary for i32{
    fn summarise(&self)->String{
        "I'm in i32".to_string()
    }
}

fn notify(name: &str, thing: &impl Summary){
    println!("notify fn says:");
    println!("{} = {}", name, thing.summarise());
    println!("{} defaulted_fn = {}", name, thing.defaulted_fn());
    println!("{} something else = {}",name,  thing.something_else());
}

fn trait_bound_fn<T:Summary>(name:&str, thing: &T){
    println!("trait_bound_fn says:");
    println!("{} = {}", name, thing.summarise());
    println!("{} defaulted_fn = {}", name, thing.defaulted_fn());
    println!("{} something else = {}",name,  thing.something_else());
}

fn main() {
    let numbers = vec![4,5,6,10,3,4,5,3,2];
    let max = find_max(&numbers);
    println!("The max is {}", max);
    let numbers = vec![100,1001,300,3203];
    let max = find_max(&numbers);
    println!("The max is {}", max);

    let ref_max = generic_ref_max(&numbers);
    println!("Generic ref max should be the same as before {}", ref_max);
    println!("max ref = {:p}, normal max = {:p}, actual_max = {:p}", ref_max, &max, &numbers[3]);

    let max = generic_max(&numbers);
    let numbers = vec![22.3, 32.3, 43.2];
    println!("Generic max works on i32 = {}", max);
    let max = generic_max(&numbers);
    println!("Generic max works on f32= {}", max);

    let max = 4;
    let p = Point{x:40.0, y:32.2};
    println!("p.x = {} p.y = {}", p.x, p.y);

    let y = Point{x:"World", y: "Hello"};
    println!("y.x = {} p.y = {}", y.x, y.y);

    //let _ = y.distance(); doesn't compiles cause there's no definition for Point<i32, &str>::distance()
    let _ = p.distance();
    let swapperoonies = p.swap(y);
    println!("swapperoonies.x = {} swaperoonies.y = {}", swapperoonies.x, swapperoonies.y);

    notify("swapperoonies", &swapperoonies);
    notify("max", &max);

    trait_bound_fn("swaperoonies", &swapperoonies);
    trait_bound_fn("max",  &max);
}
