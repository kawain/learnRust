# ライブラリの作成

他の言語における「ライブラリ」や「パッケージ」のことをRustではクレートと呼びます。  
クレートの中に1つ以上のモジュールがあります。  
ライブラリ用のプロジェクトを作成する場合--libを付ける
```
$ cargo new package名 --lib
```
main.rsでなくlib.rsが作られる
```
.
├── Cargo.toml
└── src
    └── lib.rs
```
lib.rsの中身  
> testsモジュールの#[cfg(test)]という注釈は、コンパイラにcargo buildを走らせた時ではなく、cargo testを走らせた時にだけ、 テストコードをコンパイルし走らせるよう指示します
```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```
ライブラリを作成していく過程ではmain.rsがないので、プログラムの実行結果がわかりません。  
別途main.rsを作成して確認していく方法もあるのでしょうが、テストしながら作成していくのがデフォルトということでしょう。  

## テストしながら作成することを「テスト駆動開発」という

> テスト駆動開発 (TDD) とは、プログラム開発手法の一種で、プログラムに必要な各機能について、最初にテストを書き（これをテストファーストと言う）、そのテストが動作する必要最低限な実装をとりあえず行った後、コードを洗練させる、という短い工程を繰り返すスタイルである。

テスト実行(testアトリビュートがついた関数を実行する)
```
$ cargo test
```
関数を作成してテストしてみる  
pubが付いたものは他のモジュールでアクセスできる公開された・publicなという意味です。基本は非公開、privateです。
```
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add2() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```
`#[ignore]`でテストを無視できる  
testでprint内容を表示したければ　`cargo test -- --nocapture`　で実行
```
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // #[ignore]
    fn it_works() {
        let input = "let five = 5;";
        let l = Lexer::new(input.to_string());

        println!("{:?}", l);

        assert_eq!("let five = 5;", l.input);
    }
}

$ cargo test -- --nocapture

running 1 test
Lexer { input: "let five = 5;", position: 0, read_position: 1, ch: 'l' }
test lexer::tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
こういう関数を作成してテストがpassしてから
```
pub fn hello() -> String {
    format!("hello")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hello_test() {
        assert_eq!("hello", hello());
    }
}
```
main.rsを作成して使ってみる  
これはエラー(not found in this scope)
```
fn main() {
    let s = hello();
    print!("{}", s);
}
```
useを使う
```
use package名::hello;

fn main() {
    let s = hello();
    print!("{}", s);
}
```

## モジュールを分ける

main.rsがない、最初のライブラリ用のプロジェクトに戻します  
module1～というファイルを作成して、
```
.
├── Cargo.toml
└── src
    ├── lib.rs
    ├── module1.rs
    ├── module2.rs
    └── module3.rs
```
module1.rs
```
pub fn hello() -> String {
    "hello".to_string()
}
```
module2.rs
```
pub fn hello() -> String {
    "hello2".to_string()
}
```
module3.rs
```
pub fn hello() -> String {
    "hello3".to_string()
}
```
lib.rsにmod～を書いて、module1::のように呼び出す。  
これでテストパス
```
mod module1;
mod module2;
mod module3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_test3() {
        assert_eq!("hello3", module3::hello());
    }

    #[test]
    fn hello_test2() {
        assert_eq!("hello2", module2::hello());
    }

    #[test]
    fn hello_test1() {
        assert_eq!("hello", module1::hello());
    }
}
```
main.rsを作りライブラリを呼び出す  
lib.rsのmodの箇所をpub modにする
```
// mod module1;
// mod module2;
// mod module3;

use package名::module1;
use package名::module2;
use package名::module3;

fn main() {
    println!("{}", module3::hello());
    println!("{}", module2::hello());
    println!("{}", module1::hello());
}
```

lib.rsとmain.rsがあるのは同時に２つのクレートがある状態

## main.rsが作成されるbinクレート内でモジュールを作成する場合

```
$ cargo new mod_test

.
├── Cargo.toml
└── src
    └── main.rs
```
モジュールを追加  
module1.rs
```
pub fn hello1() -> String {
    "hello1".to_string()
}
```
module2.rs
```
pub fn hello2() -> String {
    "hello2".to_string()
}
```
module3.rs
```
pub fn hello3() -> String {
    "hello3".to_string()
}
```
main.rs
```
mod module1;
mod module2;
mod module3;

fn main() {
    println!("{}", module1::hello1());
    println!("{}", module2::hello2());
    println!("{}", module3::hello3());
}
```
module2からmodule1を使うような場合  
- use crate::module1; // 絶対パス
- use super::module1; // 相対パス

いずれかで読み込む
```
// use crate::module1;
use super::module1;

pub fn hello2() -> String {
    module1::hello1()
}
```

# RustでPythonのライブラリ作成

作成方法は3つくらいあります。

- Pythonのctypesを使う方法
- PyO3というクレートを使う方法
- rust-cpythonというクレートを使う方法

PyO3はgithubのstar数2.4kで有名ですが、Rustのnightly版必須なので今回はパスします。
代わりにrust-cpythonというクレートを使用します。

https://github.com/dgrunwald/rust-cpython  
https://crates.io/crates/cpython  

以前作成した逆ポーランド電卓をPythonで動かしていきます。


(1)Python側のコード app.py 作成
```
import rpnpy

a = input("逆ポーランド入力：")
b = rpnpy.rpn(a)
print(b)
```
入力はPythonのinputから文字列を取得。
それを作成したライブラリの関数に渡し、返り値を文字列で取得して表示。

(2)新規にライブラリ用のプロジェクトを作成する(rpnpyという名前にした)
```
$ cargo new rpnpy --lib
```
(3)Cargo.tomlに以下を追加
```
[lib]
name = "rpnpy"
crate-type = ["cdylib"]

[dependencies.cpython]
version = "0.5"
features = ["extension-module"]
```
(4)lib.rsにrust-cpythonの説明にあるコードをコピペ(雛形にする)
```
use cpython::{PyResult, Python, py_module_initializer, py_fn};

// add bindings to the generated python module
// N.B: names: "rust2py" must be the name of the `.so` or `.pyd` file
py_module_initializer!(rust2py, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "sum_as_string", py_fn!(py, sum_as_string_py(a: i64, b:i64)))?;
    Ok(())
});

// logic implemented as a normal rust function
fn sum_as_string(a:i64, b:i64) -> String {
    format!("{}", a + b).to_string()
}

// rust-cpython aware function. All of our python interface could be
// declared in a separate module.
// Note that the py_fn!() macro automatically converts the arguments from
// Python objects to Rust values; and the Rust return value back into a Python object.
fn sum_as_string_py(_: Python, a:i64, b:i64) -> PyResult<String> {
    let out = sum_as_string(a, b);
    Ok(out)
}
```
(5)以下のように書き換えた
```
use cpython::{py_fn, py_module_initializer, PyResult, Python};

py_module_initializer!(rpnpy, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "rpn", py_fn!(py, rpn_py(a: String)))?;
    Ok(())
});

fn rpn(input: String) -> String {
    // 入力した文字列をスペースで分解したベクタ
    let v: Vec<&str> = input.split_whitespace().collect();
    // 数字を入れるベクタ
    let mut v_num: Vec<f64> = Vec::new();

    for s in &v {
        match s.parse::<f64>() {
            Ok(n) => v_num.push(n),
            Err(_) => {
                // 数字でないので演算子をチェックする
                // sは&&strになっているので、参照を1つ取り除き&strにする
                // popした要素の順番に注意
                // 3 4 + 1 2 - * の場合、
                // 3 4 + で 3 + 4
                // 1 2 - で 1 - 2 = -1
                match *s {
                    "+" => {
                        let n1 = v_num.pop().unwrap();
                        let n2 = v_num.pop().unwrap();
                        v_num.push(n2 + n1);
                    }
                    "-" => {
                        let n1 = v_num.pop().unwrap();
                        let n2 = v_num.pop().unwrap();
                        v_num.push(n2 - n1);
                    }
                    "*" => {
                        let n1 = v_num.pop().unwrap();
                        let n2 = v_num.pop().unwrap();
                        v_num.push(n2 * n1);
                    }
                    "/" => {
                        let n1 = v_num.pop().unwrap();
                        let n2 = v_num.pop().unwrap();
                        v_num.push(n2 / n1);
                    }
                    _ => {
                        return format!("無効な文字があります");
                    }
                }
            }
        };
    }

    let y = (v_num[0] * 100.0).round() / 100.0;
    format!("答え：{}", y)
}

fn rpn_py(_: Python, a: String) -> PyResult<String> {
    let out = rpn(a);
    Ok(out)
}
```
(6)リリースビルドする
```
$ cargo build --release
```
(7)実行ファイルをPythonのある場所に持ってきてリネーム

Linuxの場合  
target/release/librpnpy.so をlibを取って rpnpy.so にリネーム

Windowsの場合  
\target\release\rpnpy.dll を rpnpy.pyd にリネーム

(8)app.py を実行してみる
