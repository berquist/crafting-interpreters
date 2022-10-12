fn main() {
    let mut a = 1;
    {
        let a = a + 2;
        println!("{}", a);
    }
    println!("{}", a);
}
