fn main() {
    let a = "global";
    // error[E0434]: can't capture dynamic environment in a fn item
    //  --> closure.rs:4:24
    //   |
    // 4 |         println!("{}", a);
    //   |                        ^
    //   |
    //   = help: use the `|| { ... }` closure form instead

    // error: aborting due to previous error
    // fn show_a() {
    //     println!("{}", a);
    // }
    let show_a = || {
        println!("{}", a);
    };
    show_a();
    let a = "block";
    show_a();
}
