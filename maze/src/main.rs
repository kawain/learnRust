use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    println!("迷路の大きさを5以上で入力(奇数でお願いします)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(v) => {
            if v % 2 == 0 && v >= 5 {
                println!("最低5以上の奇数でお願いします");
            } else {
                let maze2 = anahori(v);
                display(&maze2);
            }
        }
        Err(_) => println!("数値を入力してください"),
    };
}

fn anahori(n: usize) -> Vec<Vec<u8>> {
    // 最初に全てのマスを1(壁)として初期化する
    // 要素が全部1の2次元ベクタ作成
    let mut maze: Vec<Vec<u8>> = vec![vec![1; n]; n];

    // 最初の地点を選択
    // 外側のマスは除いて、内側のマスの奇数座標のマスをランダムに選択
    // もし n==9 の場合 0..8x0..8 の正方形ができる
    // 外を除くと 1,2,3,4,5,6,7 のうち、1,3,5,7 からランダムに選ぶことになる
    // イテレータから奇数要素抽出
    let odd = (0..n).filter(|x| x % 2 != 0).collect::<Vec<_>>();
    // このベクタから2回chooseして最初の地点とする
    // デフォルトの乱数生成器を初期化します
    let mut rng = thread_rng();
    let mut row = *odd.choose(&mut rng).unwrap();
    let mut col = *odd.choose(&mut rng).unwrap();
    // その地点を 0（通路）にします
    maze[row][col] = 0;

    // 地点用スタック
    let mut stack = Vec::new();
    // 現在の地点として保持
    stack.push((row, col));

    loop {
        // スタックの推移
        // println!("{:?}",stack);
        // stack 全部戻ったら終わり
        if stack.len() == 0 {
            break;
        }

        // 2つ先地点が 1（壁）なら ok
        let f = move_maze(&mut maze, row, col);
        if f.0 == false {
            let p = stack.pop().unwrap();
            row = p.0;
            col = p.1;
            continue;
        }

        row = f.1;
        col = f.2;

        stack.push((row, col));
    }

    maze
}

fn move_maze(maze: &mut Vec<Vec<u8>>, row: usize, col: usize) -> (bool, usize, usize) {
    let mut lst = vec!["up", "down", "left", "right"];
    let maze_len = maze.len();

    loop {
        // ランダムに選択
        let d = lst.choose(&mut rand::thread_rng()).unwrap();

        // 1つ先用(マイナスでも大丈夫なようにキャストしておく)
        let mut r1 = row as i32;
        let mut c1 = col as i32;

        // 2つ先用(マイナスでも大丈夫なようにキャストしておく)
        let mut r2 = row as i32;
        let mut c2 = col as i32;

        if *d == "up" {
            r1 = r1 - 1;
            r2 = r2 - 2;
        } else if *d == "down" {
            r1 = r1 + 1;
            r2 = r2 + 2;
        } else if *d == "left" {
            c1 = c1 - 1;
            c2 = c2 - 2;
        } else if *d == "right" {
            c1 = c1 + 1;
            c2 = c2 + 2;
        }

        // 2つ先地点が 1（壁）なら ok
        if r2 < maze_len as i32 && c2 < maze_len as i32 && r2 >= 0 && c2 >= 0 {
            if maze[r2 as usize][c2 as usize] == 1 {
                maze[r1 as usize][c1 as usize] = 0;
                maze[r2 as usize][c2 as usize] = 0;
                return (true, r2 as usize, c2 as usize);
            }
        }

        // ループ毎にdを削除
        lst = lst.iter().filter(|x| x != &d).cloned().collect();
        // 空になれば抜ける
        if lst.len() == 0 {
            break;
        }
    }

    (false, 0, 0)
}

fn display(maze: &Vec<Vec<u8>>) {
    // 画面表示
    for v1 in maze.iter() {
        for v2 in v1.iter() {
            if *v2 == 1 {
                print!("■");
            } else {
                print!("□");
            }
        }
        print!("\n");
    }

    // ファイルに書き込む
    let mut text = String::new();
    for v1 in maze.iter() {
        for v2 in v1.iter() {
            if *v2 == 1 {
                text.push('#');
            } else {
                text.push(' ');
            }
        }
        text.push('\n');
    }
    let mut buffer = File::create("maze.txt").unwrap();
    buffer.write(&text.as_bytes()).unwrap();
}
