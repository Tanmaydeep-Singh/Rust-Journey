struct Point<T, U> {
    x: T,
    y: U,
}


fn main() {
    let integer_and_float = Point { x: 5, y: 4.0 };
    
    print!("{},{}",integer_and_float.x, integer_and_float.y);
   
}