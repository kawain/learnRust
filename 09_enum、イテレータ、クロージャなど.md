# メモリの 4 領域

参考url:
https://brain.cc.kogakuin.ac.jp/~kanamaru/lecture/MP/final/part06/node8.html

- テキスト領域：機械語に翻訳されたプログラムが格納される。
- 静的領域：グローバル変数や文字列リテラルなどが置かれる。
- スタック領域：全て既知の固定サイズのデータ。プリミティブ型が置かれる。
- ヒープ領域：コンパイル時にサイズがわからなかったり、サイズが可変のデータ(String、ベクタなどのコレクション型、構造体など)。  
ヒープへのデータアクセスは、スタックのデータへのアクセスよりも低速だが、領域は大きい。

## Box型

スタックではなくヒープにデータを格納したい時やここに格納するしかない時に使う型。  
```
fn main() {
    // ヒープに格納
    let a = Box::new(5);
    let b = a;
    // ムーブするのでエラー
    // println!("{}", a);
    println!("{}", b);
}
```
ボックス化された値に対してパターンマッチを行う場合は、参照外しする必要があります
```
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let boxed_point = Box::new(Point { x: 0, y: 0 });
    match *boxed_point {
        Point { x, y } => println!("Point is at ({}, {})", x, y),
    }
}
```

## typeエイリアス

type キーワードを用いることで他の型へのエイリアスを宣言することができます

```
type MojiRetsuGata = String;

fn main() {
    let x: MojiRetsuGata = "Hello".to_string();
    println!("{}", x);
}
```
型が長くなった時に使うのが有効
```
type VecVec = Vec<Vec<String>>;

fn main() {
    let mut a = Vec::new();
    a.push(vec!["hello".to_string(), "world".to_string()]);
    some_fn(&mut a);

    for v1 in a.iter() {
        for v2 in v1.iter() {
            println!("{}", v2);
        }
    }
}

fn some_fn(v: &mut VecVec) {
    v.push(vec!["こんにちは".to_string(), "世界".to_string()]);
}
```

## 列挙型(enum)

異なる種類の値を一つの型にする

これは他言語にもある単純なenumの例
```
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Signal {
    Red,
    Yellow,
    Green,
}

fn can_you_pass(a: &Signal) -> bool {
    match a {
        Signal::Red => false,
        Signal::Yellow | Signal::Green => true,
    }
}

fn main() {
    let s = Signal::Yellow;

    if can_you_pass(&s) {
        println!("{:?}なので通過します", s);
    } else {
        println!("{:?}なので通過しません", s);
    }
}
```
Rustのenumは強力なので構造体などのデータも持てる
```
#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: &WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

fn main() {
    let mut v: Vec<WebEvent> = Vec::new();

    v.push(WebEvent::KeyPress('x'));
    v.push(WebEvent::Paste("my text".to_string()));
    v.push(WebEvent::Click { x: 20, y: 80 });
    v.push(WebEvent::PageLoad);
    v.push(WebEvent::PageUnload);

    for w in &v {
        inspect(w);
    }

    println!("{:?}", v);
}
```

## アトリビュート

```
#[test]

または以下のように:

#![test]
```

２つの違いは ! に有ります、 ! はアトリビュートが適用されるものを変更します:

```
#[foo]
struct Foo;

mod bar {
    #![bar]
}
```

modはモジュールを定義するキーワード(後述)  
> #[foo] アトリビュートは次のアイテムに適用され、この場合は struct 宣言に適用されます。  
> #![bar] アトリビュートは #![bar] アトリビュートを囲んでいるアイテムに適用され、この場合は mod 宣言に適用されます。  
> その他の点については同じであり、どちらも適用されたアイテムの意味を変化させます。

### `#[allow(dead_code)]`

コンパイラーは、未使用の関数について警告しますが、これを無効にする

### `#[derive(Debugなど)]`

> deriveアトリビュートを用いることで型に対して特定のトレイトの標準的な実装を提供する機能があります。  
> より複雑なことを行わせたい場合には、同名のトレイトを手動で実装することも可能です。

以下はderive可能なトレイトの一覧です(トレイトは後述)。

- 型の比較に関連するトレイト: Eq, PartialEq, Ord, PartialOrd
- Clone, これはコピーによって&TからTを作成するトレイト
- Copy, ムーブではなくコピーします
- Hash, これは&Tからハッシュ値を計算するためのトレイト
- Default, これは空っぽのインスタンスを作成するためのトレイト
- Debug, これは{:?}フォーマッタを利用して値をフォーマットするためのトレイト

# イテレータ

イテレータとは .next() メソッドを繰り返し呼び出すことができ、その都度順番に値を返すものです。

```
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut i = v.iter();
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
}
```

基本的なベクタのループを例にします。

> ベクタで、ある値に対して for を用いて以下の様な3つの方法でイテレートすることができます

この場合イテレートしているだけでイテレータではない。
```
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("{} への参照「&i32型」", i);
    }

    println!("{:?}", v);

    for i in &mut v {
        // 変更できるが、参照外しが必要
        *i += 1;
        println!("{} への可変の参照「&mut i32型」", i);
    }

    println!("{:?}", v);

    for i in v {
        println!("{} 値そのものなので所有権を失う「i32型」", i);
    }

    // ムーブしているのでエラー
    // println!("{:?}", v);
}
```
以上はベクタをそのまま回している例です。  
この使い方だけでしたら問題ないですが、回しながら何らかの方法で加工するなどを試みる場合、
ベクタをイテレータに変換した方がベターです。

イテレータを得るための3つのメソッド
- iter()
- iter_mut()
- into_iter()
```
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    for i in v.iter() {
        // 要素が&i32
        println!("{}", i);
    }

    println!("{:?}", v);

    for i in v.iter_mut() {
        // 要素が&mut i32
        *i += 1;
        println!("{}", i);
    }

    println!("{:?}", v);

    for i in v.into_iter() {
        // 要素がi32
        println!("{}", i);
    }

    // ムーブしているのでエラー
    // println!("{:?}", v);
}
```

## イテレータのアダプターとコンシューマー(消費者)

```
fn main() {
    let mut i = 1..6;//イテレータ
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
}
```
### イテレータアダプタ map
```
fn main() {
    //         Iterator  Adapter
    //             |       |
    let my_map = (1..6).map(|x| x * x);
    println!("{:?}", my_map);

}
```
map内の`|x| x + 1`はクロージャと呼ばれる匿名関数(pythonならlambda)の書き方です
```
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    for i in v.iter().map(|x| x + 1) {
        println!("{}", i);
    }

    println!("{:?}", v);
}
```
ループして値は+1されますが、v自体は更新していません。

### 消費者はイテレータを取ってイテレータ以外のものを返し、その過程でイテレータを消費する

```
fn main() {
    //                    Iterator  Adapter       Consumer
    //                        |       |              |
    let my_squares: Vec<_> = (1..6).map(|x| x * x).collect();
    println!("{:?}", my_squares);
}
```
collect() はイテレータが渡す沢山の値を全て受け取り、その結果をコレクションとして返します
```
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let v2 = v.iter().map(|x| x + 1).collect::<Vec<i32>>();

    println!("{:?}", v2);
}
```
> ::<> 構文で型ヒント(type hint)を与え、整数型のベクタが欲しいと伝えることができます。  
> かといって常に型をまるごとを書く必要はありません。 _ を用いることで部分的に推論してくれます。
```
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let v2 = v.iter().map(|x| x + 1).collect::<Vec<_>>();

    println!("{:?}", v2);
}
```
これでもいい
```
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let v2: Vec<_> = v.iter().map(|x| x + 1).collect();

    println!("{:?}", v2);
}
```

# クロージャ

クロージャは以下のような見た目の匿名関数

```
fn main() {
    let plus_one = |x: i32| x + 1;
    let a = plus_one(10);
    println!("{}", a);
}
```
`{ }`で複数行にできる
```
fn main() {
    let plus_two = |x| {
        let mut result: i32 = x;
        result += 1;
        result += 1;
        result
    };

    let a = plus_two(10);
    println!("{}", a);
}
```
引数なしの場合
```
fn main() {
    let hello = || "World!";
    let a = hello();
    println!("{}", a);
}
```
numがスコープに入る
```
fn main() {
    let num = 5;
    let plus_num = |x| x + num;
    let a = plus_num(10);
    println!("{}", a);
}
```
クロージャを返す場合Box型に入れる
```
fn inc() -> Box<dyn FnMut() -> i32> {
    let mut num = 0;

    Box::new(move || {
        num += 1;
        num
    })
}

fn main() {
    let mut i1 = inc();
    println!("{}", i1());
    println!("{}", i1());
    println!("{}", i1());

    let mut i2 = inc();
    println!("{}", i2());
    println!("{}", i2());
    println!("{}", i2());

    println!("{}", i1());
    println!("{}", i2());
}
```
dynとはトレイトオブジェクトであることを明示するキーワード(トレイトとかトレイトオブジェクトは後述)  
FnMutはクロージャの3つのトレイトの一つ  
- Fn()
- FnMut()
- FnOnce()

moveはnumをムーブしてクロージャが所有者になる

### 普通の関数を引数に渡す場合

「fn(f64, f64) -> f64」型の関数を渡す  
fn型は、関数ポインタと呼ばれます
```
fn calc_add(a: f64, b: f64) -> f64 {
    a + b
}

fn calc_sub(a: f64, b: f64) -> f64 {
    a - b
}

fn calc_mul(a: f64, b: f64) -> f64 {
    a * b
}

fn calc_div(a: f64, b: f64) -> f64 {
    a / b
}

fn calc(a: f64, b: f64, f: fn(f64, f64) -> f64) -> f64 {
    f(a, b)
}

fn main() {
    let a = 9.0;
    let b = 3.0;

    let i1 = calc(a, b, calc_add);
    let i2 = calc(a, b, calc_sub);
    let i3 = calc(a, b, calc_mul);
    let i4 = calc(a, b, calc_div);

    println!("{}", i1);
    println!("{}", i2);
    println!("{}", i3);
    println!("{}", i4);
}
```

### クロージャを引数に渡す場合

```
fn calc<F>(a: f64, b: f64, f: F) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    f(a, b)
}

fn main() {
    let a = 9.0;
    let b = 3.0;

    let i1 = calc(a, b, |x, y| x + y);
    let i2 = calc(a, b, |x, y| x - y);
    let i3 = calc(a, b, |x, y| x * y);
    let i4 = calc(a, b, |x, y| x / y);

    println!("{}", i1);
    println!("{}", i2);
    println!("{}", i3);
    println!("{}", i4);
}
```
`<F>`はジェネリック型の型パラメータになります。

## ジェネリック

型と関数の機能をより汎用的に使えるようにするための機能

もしもこのようなコードであれば、
```
fn calc_add_f64(a: f64, b: f64) -> f64 {
    a + b
}

fn calc_add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn calc_add_u8(a: u8, b: u8) -> u8 {
    a + b
}

fn calc_add_i8(a: i8, b: i8) -> i8 {
    a + b
}

fn main() {
    let a1: f64 = 9.0;
    let b1: f64 = 3.0;
    let i1 = calc_add_f64(a1, b1);
    println!("{}", i1);

    let a2: i32 = 9;
    let b2: i32 = 3;
    let i2 = calc_add_i32(a2, b2);
    println!("{}", i2);

    let a3: u8 = 9;
    let b3: u8 = 3;
    let i3 = calc_add_u8(a3, b3);
    println!("{}", i3);

    let a4: i8 = 9;
    let b4: i8 = 3;
    let i4 = calc_add_i8(a4, b4);
    println!("{}", i4);
}
```
こう書いた方が効率的  
Tは抽象的な型でジェネリックな型という。何でもありという感じです。  
しかしこのコードはエラーです。
```
fn calc_add<T>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let a1: f64 = 9.0;
    let b1: f64 = 3.0;
    let i1 = calc_add(a1, b1);
    println!("{}", i1);

    let a2: i32 = 9;
    let b2: i32 = 3;
    let i2 = calc_add(a2, b2);
    println!("{}", i2);

    let a3: u8 = 9;
    let b3: u8 = 3;
    let i3 = calc_add(a3, b3);
    println!("{}", i3);

    let a4: i8 = 9;
    let b4: i8 = 3;
    let i4 = calc_add(a4, b4);
    println!("{}", i4);
}
```
calc_addでT型を足しているので、Tは足し算できる機能がある型でないといけないということです。  
型には足し算できるものやできないものがあります。

例えば&str型は足し算できません。
```
fn main() {
    let s1: &str = "Hello,";
    let s2: &str = "World!";

    println!("{}", s1 + s2);
}
```

### ジェネリックの境界、トレイト境界

ジェネリック型に対して「このトレイトを実装していなければならない」という制約を課すもの

足し算できる機能はAddというトレイトで標準に用意されていて、これを実装している型のみ受付けるジェネリックの関数ができます。  
トレイト境界の書き方は3種類あるのですが、where節を使って書く例です。
```
use std::ops::Add;

fn calc_add<T>(a: T, b: T) -> T::Output
where
    T: Add,
{
    a + b
}

fn main() {
    let a1: f64 = 9.0;
    let b1: f64 = 3.0;
    let i1 = calc_add(a1, b1);
    println!("{}", i1);

    let a2: i32 = 9;
    let b2: i32 = 3;
    let i2 = calc_add(a2, b2);
    println!("{}", i2);

    let a3: u8 = 9;
    let b3: u8 = 3;
    let i3 = calc_add(a3, b3);
    println!("{}", i3);

    let a4: i8 = 9;
    let b4: i8 = 3;
    let i4 = calc_add(a4, b4);
    println!("{}", i4);
}
```

## クロージャを引数に渡す場合に戻ります

クロージャは普通の関数を渡す時と違い、型名はジェネリックにして、トレイと境界で範囲を指定して動きます。
```
// 第3引数はジェネリック型
fn calc<F>(a: f64, b: f64, f: F) -> f64
where
    // Fnはクロージャの3つのトレイとの一つ
    // FはFn(f64, f64) -> f64を実装するものに限るという境界
    F: Fn(f64, f64) -> f64,
{
    f(a, b)
}

fn main() {
    let a = 9.0;
    let b = 3.0;

    let i1 = calc(a, b, |x, y| x + y);
    let i2 = calc(a, b, |x, y| x - y);
    let i3 = calc(a, b, |x, y| x * y);
    let i4 = calc(a, b, |x, y| x / y);

    println!("{}", i1);
    println!("{}", i2);
    println!("{}", i3);
    println!("{}", i4);
}
```
