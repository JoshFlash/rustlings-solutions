// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

// print name and value of one variable
macro_rules! show {
    ($var: ident) => {
        println!("{} = {}", stringify!($var), $var);
    }
}

// print name and value of multiple variables
macro_rules! show {
    ( $($opt:ident),*) => {
        $( println!("{} = {}", stringify!($opt), $opt); )*
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);

    let (x, y, z) = (3, 4.0, "five");
    show!(x, y, z);
}
