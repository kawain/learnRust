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
