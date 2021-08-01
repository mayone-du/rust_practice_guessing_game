use rand::Rng;
use std::io;
fn main() {
    println!("数あてゲームスタート！");
    let secret_number: i32 = rand::thread_rng().gen();

    println!("予想を入力してください！");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("エラーが発生しました。");

    println!("あなたが予想した数: {}", guess);
}
