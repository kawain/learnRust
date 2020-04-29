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
