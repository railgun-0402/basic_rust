
fn main() {
    // 型推論
    let x = 13;
    println!("{}", x);

    // 型指定
    let x: f64 = 3.14159;
    println!("{}", x);

    // 宣言後に初期化
    let x;
    x = 0;
    println!("{}", x);

    basic_variable();
}

fn immute() {
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
}

pub fn basic_variable() {
    let x = 12; // デフォルトでは i32
    let a = 12u8;
    let b = 4.3; // デフォルトでは f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}

fn type_convert() {
    // u8とu32を混ぜるとエラーになる
    let a = 15u8;
    let b = 4u32;
    // asで数値型を変換する
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t);
}