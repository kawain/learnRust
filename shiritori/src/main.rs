use std::io;

fn main() {
    println!("ゲームスタート");
    let first_word = 'あ';
    start(first_word);
}

fn start(word: char) {
    println!("最初の文字 {}", word);

    let text = input();
    judgment(text, word);
}

fn input() -> String {
    println!("文字を入力してください");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn judgment(text: String, word: char) {
    let v: Vec<char> = text.trim().chars().collect();

    println!("{:?}", v);
    let len = v.len();

    if v[0] != word {
        println!("最初の文字が違います");
        println!("入力した最初の文字「{}」", v[0]);
        println!("もう一度入力してください");
        start(word);
        return;
    }

    if v[len - 1] == 'ん' {
        println!("最後の文字が「ん」です");
        end();
        return;
    }

    start(v[len - 1]);
}

fn end() {
    println!("ゲームオーバー");
    std::process::exit(0);
}
