# rustup, rustc, cargo

- rustup はインストーラー
- rustc はコンパイラ
- cargo はビルドシステム兼、パッケージマネージャ

### 最新バージョンへアップデート

```
$ rustup update
```

### rustup 自体のアップデート

```
$ rustup self update
```

### rustup バージョン確認

```
$ rustup -V
```

# Rust のバージョン確認 (実質 rustc コンパイラのバージョン)

```
$ rustc -V
```

## cargo を使わない場合のコンパイル＆実行　main.rs　はソースファイル

```
$ rustc main.rs
```

### コンパイルが成功したら、実行可能バイナリファイルを実行

```
$ ./main
```

※ 単純なプログラムなら rustc でコンパイルすれば十分ですが、プロジェクトが大きくなりそうなものはあらかじめ cargo で作成する

## cargoのバージョン確認

```
$ cargo -V
```

## cargoで新たなプロジェクトを作成

オプションなしのデフォルトでは binary package プロジェクトが作成される（バイナリ・クレートと呼ばれる）

※　--lib　オプションを付けるとライブラリ用のプロジェクトが作成される（ライブラリ・クレートと呼ばれる）

### 今いるフォルダをプロジェクトにする場合

```
$ cargo init
```

### フォルダを指定して作成する場合

```
$ cargo new hello_world
```

```
hello_world/
├── Cargo.toml (設定ファイル)
└── src
    └── main.rs (ソースファイル)
```

## デバッグビルド（コンパイルとビルドはほぼ同じ意味ですが、ビルドの方が広い意味で使われる）

```
$ cd hello_world/
$ cargo build
```

target/debug/hello_cargo  
(Windowsなら、 target/debug/hello_cargo.exe)  
に実行可能ファイルができます。  

```
$ exec ./target/debug/hello_world
Hello, world!
```

## デバッグビルド＆ラン（通常はこのコマンドを何回も実行して作成していく）

```
$ cargo run
```

## ビルドできるかチェックするコマンド（実行可能ファイルは生成しません）

```
$ cargo check
```

# プログラムが完成したらリリースビルドする

<span style="color:red">リリースビルドをしたら、デバッグビルドの実行ファイルよりも実行速度が上がります</span>

```
$ cargo build --release
```

```
$ cargo run --release
```

リリースビルドは、target/debugではなく、 target/releaseに実行可能ファイルを作成します。

## 下のコマンドを実行すると、target以下を削除してくれます

```
$ cargo clean
```
