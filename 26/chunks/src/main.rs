use byteorder::{BigEndian, ReadBytesExt};

fn lefts<T>(data: &[T]) -> std::iter::StepBy<std::slice::ChunksExact<'_, T>> {
    pair_up(data).step_by(2)
}

fn rights<T>(data: &[T]) -> std::iter::StepBy<std::iter::Skip<std::slice::ChunksExact<'_, T>>> {
    pair_up(data).skip(1).step_by(2)
}

fn join(bytes: &[u8]) -> u16 {
    let bytes = bytes.to_owned();
    bytes.as_slice().read_u16::<BigEndian>().unwrap_or(0)
}

fn pair_up<T>(data: &[T]) -> std::slice::ChunksExact<'_, T> {
    data.chunks_exact(2)
}

fn main() {
    let data: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let left: Vec<u16> = lefts(&data).map(|bytes| join(bytes)).collect();
    let right: Vec<u16> = rights(&data).map(|bytes| join(bytes)).collect();

    println!("Data = {:?}", data);
    println!("Left = {:#06X?}", left);
    println!("Right = {:#06X?}", right);
}
