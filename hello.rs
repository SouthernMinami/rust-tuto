fn main() {
    let num = 8;
    println!("Hello, {}!", num);

    // num = 10; 
    // cannot assign twice to immutable variable `num`
    // mutable変数はmutで宣言できる
    let mut mutable_num = 8;
    mutable_num = 10;
    println!("Mutable Num: {}!", mutable_num);
}
