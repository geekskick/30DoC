use itertools::Itertools;
use textplots::{Chart, Plot, Shape};
extern crate num;

fn make_plottable<T: num::cast::AsPrimitive<f32>>(data: &[&T]) -> Vec<(f32, f32)> {
    // Need to make the data into a vector of tuples for the plotter
    // were it's (x, y) in each tuple. In addition the graph is centred about 0
    // so I need to make sure it's shifted down the graph an appropriate amount so I can 
    // see all of the data I put in. num::cast::AsPrimitive<f32> ensures that
    // I can use the .as_() function on anything with the type T. This 
    // then means I can get the data out in the closure which constructs the tuples
    (-(((data.len()) / 2) as isize) + 1..(data.len() / 2) as isize)
        .zip(data.iter())
        .map(|(x, y): (isize, &&T)| {
            let y: T = **y;
            let rc: (f32, f32) = (x as f32, y.as_() as f32);
            rc
        })
        .collect()
}

fn main() {
    println!("Hello, world!");
    let bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let next_bytes: Vec<u8> = vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let merges: Vec<&u8> = Itertools::interleave(next_bytes.iter(), bytes.iter()).collect();

    println!(
        "{:?} and {:?} interleaved is {:?}",
        next_bytes, bytes, merges
    );

    let chained: Vec<&u8> = bytes.iter().chain(&next_bytes).collect();

    println!("{:?} chained with {:?} is {:?}", bytes, next_bytes, chained);

    let inverse_chained: Vec<&u8> = bytes.iter().rev().chain(next_bytes.iter().rev()).collect();

    println!("Inverse of {:?} is {:?}", chained, inverse_chained);

    Chart::default()
        .lineplot(Shape::Bars(&make_plottable(&chained)))
        .display();
    Chart::default()
        .lineplot(Shape::Bars(&make_plottable(&inverse_chained)))
        .display();
    Chart::default()
        .lineplot(Shape::Bars(&make_plottable(&merges)))
        .display();

    let mut s = merges.clone();
    s.sort();

    Chart::default()
        .lineplot(Shape::Bars(&make_plottable(&s)))
        .display();

    s.sort_unstable_by(| &a, &b | b.cmp(a));
    
    Chart::default()
        .lineplot(Shape::Bars(&make_plottable(&s)))
        .display();
}
