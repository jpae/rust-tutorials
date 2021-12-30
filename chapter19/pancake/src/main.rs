use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancake;

fn main() {
    Pancake::hello_macro();
    let sql = sql!(SELECT * FROM posts WHERE id=1);

}