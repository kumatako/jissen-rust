// pub：この関数が他のモジュールからアクセスできる
// 引数xの型 &mut [u32]
// &：値をポインタ経由で借用することを示す 借用→7章
// mut：値が変更可能であることを示す
// u32：32ビット符号なし整数
// [u32]：u32のスライス（一次元の配列のようなもの）
pub fn sort(x: &mut[u32], up: bool){
    if x.len() > 1{
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort(x: &mut[u32], up: bool){
    if x.len() > 1{
        compare_and_swap(x, up);
        let mid_point = x.len() / 2 ;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap(x: &mut[u32], up: bool){
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[i] > x[i + mid_point]) == up {
            x.swap(i, i + mid_point);
        }
    }
}

// 3-3-7 単体テストを書く（数値のソート）
// このモジュールはcargo testを実行した時のみコンパイルされる
#[cfg(test)]
mod tests {
    // superは親モジュール（first）
    use super::sort;
    
    // #[test]のついた関数はcargo testしたときに実行される
    #[test]
    fn sort_u32_ascending(){
        let mut x = vec![10,30,11,20,4,330,21,110];
        
        // xのスライスを作成してsortに渡す
        sort(&mut x, true);
        
        //昇順にソートされているのを確認
        assert_eq!(x, vec![4,10,11,20,21,30,110,330]);
    }
    
    #[test]
    fn sort_u32_descending(){
        let mut x = vec![10,30,11,20,4,330,21,110];
        
        sort(&mut x, false);
        
        //降順にソートされているのを確認
        assert_eq!(x, vec![330,110,30,21,20,11,10,4]);
    }
}