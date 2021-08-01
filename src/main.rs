use rand::Rng;
use std::cmp::Ordering;
use std::io;

// main()は実行時に呼ばれる特別な関数
fn main() {
    println!("数あてゲームスタート！");

    // ランダムな整数値を1~100の値で生成
    let secret_number = rand::thread_rng().gen_range(1..101);

    // breakするまでloop
    loop {
        println!("予想を入力してください！");

        // 予測する数字。String型で生成することによって、 heap内に可変長で保存出来るようにする
        let mut guess = String::new();

        // 標準入力を受付
        io::stdin()
            .read_line(&mut guess)
            // エラーハンドリング
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // エラー型の場合は続行
            Err(_) => continue,
        };

        println!("あなたが予想した数: {}", guess);

        // 答えと合うか判定
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("答えより小さいです!"),
            Ordering::Greater => println!("答えより大きいです!"),
            Ordering::Equal => {
                println!("正解！ 答えは{}でした！", secret_number);
                break;
            }
        }
    }
}
