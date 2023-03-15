fn main() {
    // exp変数を逆ポーランド記法の文字列に束縛する
    // 普通の数式の 6.1 + 5.2 * 4.3 - 3.4 / 2.5 * 1.6
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";
    
    // rpn関数を呼び出し、返り値にans変数を束縛する
    let ans = rpn(exp);
    
    // デバッグビルド時のみ、答えが正しいかチェックするマクロ
    debug_assert_eq!("26.2840", format!("{:.4}",ans));
    
    // ansは小数点以下四桁まで表示する
    println!("{} = {:.4}",exp,ans);
}

// 文字列expを受け取り、f64型の計算結果を返す
fn rpn(exp: &str) -> f64{
    // 変数stackを空のスタックに束縛する
    // stackはmutable（可変）な変数で、値の変更を防ぐ
    let mut stack = Vec::new();
    
    // expの要素をスペースで分割し、tokenをそれらに順に束縛する
    // 要素がなくなるまで繰り返す
    for token in exp.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        }else{
            //tokenが数値でないなら、演算子なのか調べる
            match token{
                //tokenが演算子ならapply2関数で計算する
                // |x, y| はクロージャ
                //引数x,yをとり、x+yを計算して答えを出す
                "+" => apply2(&mut stack, |x,y| x + y),
                "-" => apply2(&mut stack, |x,y| x - y),
                "*" => apply2(&mut stack, |x,y| x * y),
                "/" => apply2(&mut stack, |x,y| x / y),
                
                //tokenが演算子でないなら、エラーを起こして終了する
                // _ は上の+-/*以外
                _ => panic!("Unknown operator: {}",token),
            }
        }
    }
    
    // stackから数値を一つ取り出して返す。失敗したらエラーを起こして終了する
    stack.pop().expect("Stack underflow")
}

//スタックから数値を二つ取り出し、F型のクロージャfunで計算し、結果をスタックに積む
fn apply2<F>(stack: &mut Vec<f64>, fun: F)
//F型のトレイト境界
where
    F: Fn(f64, f64) -> f64,
{
    //変数yとxをスタックの最後の２要素に束縛する
    if let(Some(y), Some(x)) = (stack.pop(), stack.pop()){
        //クロージャfunで計算し、その結果に変数zを束縛する
        let z = fun(x,y);
        //zの値をスタックに積む
        stack.push(z);
    }else{
        //スタックから要素が取り出せなかった時はエラーを起こして終了する
        panic!("Stack underflow");
    }
}