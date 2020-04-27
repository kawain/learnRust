use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Point(i32, i32);

fn make_vec(arg: &str) -> Vec<Vec<char>> {
    let mut v: Vec<Vec<char>> = Vec::new();
    let r: Vec<&str> = arg.split("\n").collect();
    for v1 in r.iter() {
        if *v1 != "" {
            v.push(v1.chars().collect());
        }
    }
    v
}

fn search(arg: &Vec<Vec<char>>) -> HashMap<Point, i32> {
    // スタートとゴール地点を覚える
    let mut start = Point(0, 0);
    let mut goal = Point(0, 0);

    for (i1, v1) in arg.iter().enumerate() {
        for (i2, v2) in v1.iter().enumerate() {
            if *v2 == 'S' {
                start = Point(i1 as i32, i2 as i32);
            }
            if *v2 == 'G' {
                goal = Point(i1 as i32, i2 as i32);
            }
        }
    }

    // 動ける場所登録、ステップ回数登録、最初は未訪問-1、訪問済みは0以上
    let mut m = HashMap::new();
    for (i1, v1) in arg.iter().enumerate() {
        for (i2, v2) in v1.iter().enumerate() {
            if *v2 == ' ' || *v2 == 'S' || *v2 == 'G' {
                m.insert(Point(i1 as i32, i2 as i32), -1);
            }
        }
    }

    // スタート地点を訪問済みにする
    m.insert(start, 0);

    // 空のキュー作成
    let mut q = VecDeque::new();
    // キューに追加
    q.push_back(start);

    loop {
        // キューから取り出す
        if let Some(p) = q.pop_front() {
            // ゴールであれば探索をやめる
            if p == goal {
                break;
            }

            // 上下左右
            let up = Point(p.0 - 1, p.1);
            let down = Point(p.0 + 1, p.1);
            let left = Point(p.0, p.1 - 1);
            let right = Point(p.0, p.1 + 1);

            for t in [up, down, left, right].iter() {
                // 動ける場所かどうか？
                if let Some(_) = m.get(t) {
                    // 訪問済みならパス
                    if m[t] != -1 {
                        continue;
                    }

                    // 未訪問なら
                    m.insert(*t, m[&p] + 1);
                    // キューに追加
                    q.push_back(*t);
                }
            }
        }

        // 空なら抜ける
        if q.len() == 0 {
            break;
        }
    }
    m
}

fn result_display(arg1: &mut Vec<Vec<char>>, arg2: &HashMap<Point, i32>) {
    // ゴールの場所から見ていく
    let mut goal = Point(0, 0);

    for (i1, v1) in arg1.iter().enumerate() {
        for (i2, v2) in v1.iter().enumerate() {
            if *v2 == 'G' {
                goal = Point(i1 as i32, i2 as i32);
                break;
            }
        }
    }

    let mut step = arg2[&goal];
    let mut p = goal;
    let mut shortest = Vec::new();
    shortest.push(goal);

    while step >= 0 {
        // 上下左右
        let up = Point(p.0 - 1, p.1);
        let down = Point(p.0 + 1, p.1);
        let left = Point(p.0, p.1 - 1);
        let right = Point(p.0, p.1 + 1);

        for t in [up, down, left, right].iter() {
            if let Some(n) = arg2.get(&t) {
                if *n == step - 1 {
                    shortest.push(*t);
                    p = *t;
                }
            }
        }

        step -= 1;
    }

    for v in shortest.iter() {
        arg1[v.0 as usize][v.1 as usize] = '+';
    }

    for v1 in arg1.iter() {
        for v2 in v1.iter() {
            print!("{}", v2);
        }
        print!("\n");
    }
}

fn main() {
    let mut file = File::open("maze.txt").unwrap();
    let mut maze = String::new();
    file.read_to_string(&mut maze).unwrap();

    println!("{}", &maze);
    let mut v = make_vec(&maze);
    let m = search(&v);
    result_display(&mut v, &m);
}
