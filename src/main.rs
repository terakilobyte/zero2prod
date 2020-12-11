fn main() {
    println!("Hello, world!");
    println!("Hell yeah it works!");
    println!("this should cause the previous line to error")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2, 2);
        println!("foo!@")
    }
}
