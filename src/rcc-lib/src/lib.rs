#![feature(euclidean_division)]

type RccDouble = f64;
type RccInt = i32;

mod math;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
