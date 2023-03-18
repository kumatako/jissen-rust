// pub：この関数が他のモジュールからアクセスできる
// 引数xの型 &mut [u32]
// &：値をポインタ経由で借用することを示す 借用→7章
// mut：値が変更可能であることを示す
// u32：32ビット符号なし整数
// [u32]：u32のスライス（一次元の配列のようなもの）
use super::SortOrder;

// 成功時にはOK(())を、失敗時にはErr(文字列)を返す
pub fn sort<T: Ord>(x: &mut[T], order: &SortOrder) -> Result<(),String>{
    if x.len().is_power_of_two(){
        match *order {
            SortOrder::Ascending => do_sort(x, true),
            SortOrder::Descending => do_sort(x, false),
        };
        Ok(())
    }else{
        Err(format!("The length of x is not a power of two. (x.len(): {}",x.len()))
    }
}

fn do_sort<T: Ord>(x: &mut[T], up: bool){
    if x.len() > 1{
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true);
        do_sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort<T: Ord>(x: &mut[T], up: bool){
    if x.len() > 1{
        compare_and_swap(x, up);
        let mid_point = x.len() / 2 ;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap<T: Ord>(x: &mut[T], up: bool){
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
    // superは親モジュール（second）
    use super::sort;
    use crate::SortOrder::*;
    
    // #[test]のついた関数はcargo testしたときに実行される
    #[test]
    fn sort_u32_ascending(){
        // xに型注釈Vec<u32>をつける
        let mut x: Vec<u32> = vec![10,30,11,20,4,330,21,110];
        
        // xのスライスを作成してsortに渡す
        assert_eq!(sort(&mut x, &Ascending),Ok(()));
        
        //昇順にソートされているのを確認
        assert_eq!(x, vec![4,10,11,20,21,30,110,330]);
    }
    
    #[test]
    fn sort_u32_descending(){
        let mut x: Vec<u32> = vec![10,30,11,20,4,330,21,110];
        
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        
        //降順にソートされているのを確認
        assert_eq!(x, vec![330,110,30,21,20,11,10,4]);
    }
    
    #[test]
    fn sort_str_ascending(){
        let mut x = vec!["Rust","is","fast","and","memory-efficient","with","no","GC"];
        assert_eq!(sort(&mut x,&Ascending),Ok(()));
        assert_eq!(x, vec!["GC","Rust","and","fast","is","memory-efficient","no","with"]);
    }
    
    #[test]
    fn sort_str_descending(){
        let mut x = vec!["Rust","is","fast","and","memory-efficient","with","no","GC"];
        assert_eq!(sort(&mut x,&Descending),Ok(()));
        assert_eq!(x, vec!["with","no","memory-efficient","is","fast","and","Rust","GC"]);
    }
    
    #[test]
    fn sort_to_fail(){
        // 要素数が2のべき乗になっていない
        let mut x = vec!["10","30","11"];
        assert!(sort(&mut x, &Ascending).is_err());
    }
}