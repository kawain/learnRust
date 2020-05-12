## spawn

```
use std::thread;
use std::time::Duration;

fn heavy(i: u32) {
    println!("重い処理 {}", i);
    thread::sleep(Duration::from_millis(500));
}

fn main() {
    for i in 1..=10 {
        heavy(i)
    }

    for i in 1..=10 {
        heavy(i)
    }
}
```
spawnで新規スレッドを生成
```
use std::thread;
use std::time::Duration;

fn heavy(i: u32) {
    println!("重い処理 {}", i);
    thread::sleep(Duration::from_millis(500));
}

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            heavy(i);
        }
    });

    for i in 1..=10 {
        heavy(i)
    }

    handle.join().unwrap();
}
```
メインスレッドの値を使うにはmove
```
use std::thread;
use std::time::Duration;

fn heavy(i: u32, s: &str) {
    println!("重い処理 {} {}", i, s);
    thread::sleep(Duration::from_millis(500));
}

fn main() {
    let s = "メインスレッドの値";

    let handle = thread::spawn(move || {
        for i in 1..=10 {
            heavy(i, s);
        }
    });

    for i in 1..=10 {
        heavy(i, s)
    }

    handle.join().unwrap();
}
```

## チャンネル

スレッド間のメッセージ受け渡し

```
use std::thread;
use std::time::Duration;

fn heavy(i: u32) -> u32 {
    thread::sleep(Duration::from_millis(50));
    i * 2
}

fn main() {
    let handle = thread::spawn(|| {
        let mut n = 0;
        for i in 1..=10 {
            n += heavy(i);
            println!("{}", n);
        }
    });

    handle.join().unwrap();
}
```
nをメインスレッドで受けるには
```
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn heavy(i: u32) -> u32 {
    thread::sleep(Duration::from_millis(50));
    i * 2
}

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut n = 0;
        for i in 1..=10 {
            n += heavy(i);
            println!("{}", n);
        }
        tx.send(n).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("nは{}", received);
}
```
ループでも送れる
```
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn heavy(i: u32) -> u32 {
    thread::sleep(Duration::from_millis(50));
    i * 2
}

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=10 {
            let n = heavy(i);
            tx.send(n).unwrap();
        }
    });

    for received in rx {
        println!("メインスレッド側 {}", received);
    }
}
```

## randクレート

https://crates.io/crates/rand

Add this to your Cargo.toml:
```
[dependencies]
rand = "0.7"
```
よく使いそうなもの
```
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();

    let number = rng.gen_range(1, 11);
    println!("1から10までのランダムな数字 {}", number);

    let mut v = vec![1, 2, 3, 4, 5];
    let c = v.choose(&mut rng).unwrap();
    println!("vからランダムに1つ選択 {}", c);

    v.shuffle(&mut rng);
    println!("vをシャッフル {:?}", v);
}
```

## テキストファイル入出力

ファイルがなければ新規作成して書き込む  
ファイルがあれば上書き
```
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let b = "私は誰でもない！あなたは誰？
あなたも誰でもないの？
なら、私たちは組だね、何も言わないで！
あの人たちは、私たちを追放するでしょう。わかりますよね？

誰かでいるなんて侘しいじゃない！
カエルみたいで公すぎるじゃない。
自分の名を長い1日に告げるのなんて。
感服するような沼地にね！"
        .as_bytes();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("poem.txt")
        .unwrap();

    file.write_all(b).unwrap();
}
```
ファイルがなければ新規作成して書き込む  
ファイルがあれば追加して書き込む
```
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let b = "私は誰でもない！あなたは誰？
あなたも誰でもないの？
なら、私たちは組だね、何も言わないで！
あの人たちは、私たちを追放するでしょう。わかりますよね？

誰かでいるなんて侘しいじゃない！
カエルみたいで公すぎるじゃない。
自分の名を長い1日に告げるのなんて。
感服するような沼地にね！"
        .as_bytes();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("poem.txt")
        .unwrap();

    file.write_all(b).unwrap();
}
```
読み込む
```
use std::fs;

fn main() {
    let foo = fs::read_to_string("poem.txt").unwrap();
    println!("{}", foo);
}
```
読む場合 BufReader を使用する方が早いらしい
```
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let file = File::open("poem.txt").unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    println!("{}", contents);
}
```
書く場合も BufWriter を使用する方が早いらしい
```
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn main() {
    let b = "私は誰でもない！あなたは誰？
あなたも誰でもないの？
なら、私たちは組だね、何も言わないで！
あの人たちは、私たちを追放するでしょう。わかりますよね？

誰かでいるなんて侘しいじゃない！
カエルみたいで公すぎるじゃない。
自分の名を長い1日に告げるのなんて。
感服するような沼地にね！"
        .as_bytes();

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("poem.txt")
        .unwrap();

    let mut f = BufWriter::new(file);

    f.write_all(b).unwrap();
}
```

## panic!マクロ

わざとエラーを起こします

```
fn panic_test(n: i32) -> bool {
    // わざとパニック
    if n > 10 {
        panic!("クラッシュして炎上");
    }
    true
}

fn main() {
    let f = panic_test(11);
    println!("{}", f);
}
```

## エラーハンドリング

Option型とResult型の復習と値の取り出し方

先程のコードでpanicを起こさないように考えてみます

### 値がない可能性がある場合はOption型を用いる

```
enum Option<T> {
    None,
    Some(T),
}
```
Option型で返す
```
fn panic_test(n: i32) -> Option<bool> {
    if n > 10 {
        return None;
    }
    Some(true)
}

fn main() {
    let f = panic_test(11);
    // そのまま見るとSomeで包まれているかNone
    println!("{:?}", f);
    //マッチで中身を取り出す
    match f {
        Some(v) => println!("{}", v),
        None => println!("ありませんでした"),
    }
}
```

### エラーが発生する可能性がある場合はResult型を用いる

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
Result型で返す
```
fn panic_test(n: i32) -> Result<bool, String> {
    if n > 10 {
        return Err("クラッシュして炎上".to_string());
    }
    Ok(true)
}

fn main() {
    let f = panic_test(11);
    // そのまま見るとOkかErrで包まれている
    println!("{:?}", f);
    //マッチで中身を取り出す
    match f {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
}
```

### refとref mutでパターンに参照を生成

matchアームパターンで変数を生成すると、値の所有権が奪われる
```
fn main() {
    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(name) => println!("Found a name: {}", name),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);
}
```
変数の前にrefキーワードを使用
```
fn main() {
    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);
}
```
可変参照を生成するには、&mutの代わりにref mut
```
fn main() {
    let mut robot_name = Some(String::from("Bors"));

    match robot_name {
        // name は参照なので、*name参照外しで値を更新
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);
}
```

### マッチでifを使う場合

```
fn main() {
    let num = Some(4);

    match num {
        // 5未満です: {}
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}
```

以上

https://doc.rust-jp.rs/book/second-edition/ch18-03-pattern-syntax.html
