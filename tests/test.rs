#[cfg(test)]
mod tests {

    use rinput::{input, rin};

    #[test]
    fn input_test() {
        input!(a: i32, mut c: char, s: String);
        println!("'{a}' '{c}' '{s}'");

        let a: i32 = rin.read();
        let c = rin.read_char();
        let s = rin.read_str();
        println!("'{a}' '{c}' '{s}'");
    }
}
