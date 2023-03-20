// pub：この関数が他のモジュールからアクセスできる
// 引数xの型 &mut [u32]
// &：値をポインタ経由で借用することを示す 借用→7章
// mut：値が変更可能であることを示す
// u32：32ビット符号なし整数
// [u32]：u32のスライス（一次元の配列のようなもの）
use super::SortOrder;
use std::cmp::Ordering;
use rayon;

const PARALLEL_THRESHOLD: usize = 4096;

pub fn sort_by<T,F>(x: &mut[T], comparator: &F) -> Result<(),String>
    where T: Send,
        F: Sync + Fn(&T, &T) -> Ordering
{
    if x.len().is_power_of_two(){
        do_sort(x, true, comparator);
        Ok(())
    } else{
        Err(format!("The length of x is not a power of two. (x.len(): {}",x.len()))
    } 
    
}

// 成功時にはOK(())を、失敗時にはErr(文字列)を返す
pub fn sort<T: Ord + Send>(x: &mut[T], order: &SortOrder) -> Result<(),String>{
    match *order {
        SortOrder::Ascending => sort_by(x, &|a,b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a,b| b.cmp(a)),
    }
}

fn do_sort<T, F>(x: &mut[T], forward: bool, comparator: &F)
    where T: Send,
         F: Sync + Fn(&T,&T) -> Ordering
{
    if x.len() > 1{
        let mid_point = x.len() / 2;
        
        let (first, second) = x.split_at_mut(mid_point);
        
        // xの分割後の要素数をしきい値と比較する
        if mid_point >= PARALLEL_THRESHOLD {
            // 閾値以上なら並列処理
            rayon::join(|| do_sort(first, true, comparator),
                || do_sort(second, false, comparator));
        }else{
            // 閾値未満なら順次処理
            do_sort(first, true, comparator);
            do_sort(second, false, comparator);
        }
        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut[T], forward: bool, comparator: &F)
    where T: Send,
        F: Sync + Fn(&T,&T) -> Ordering
{
    if x.len() > 1{
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2 ;
        
        let (first, second) = x.split_at_mut(mid_point);
        
        if mid_point >= PARALLEL_THRESHOLD{
            rayon::join(|| sub_sort(first, forward, comparator),
                || sub_sort(second, forward, comparator));
        }else{
            sub_sort(first, forward, comparator);
            sub_sort(second, forward, comparator);
        }
    }
}

fn compare_and_swap<T, F>(x: &mut[T], forward: bool, comparator: &F)
    where F: Fn(&T,&T) -> Ordering
{
    let swap_condition = if forward{
        Ordering::Greater
    }else{
        Ordering::Less
    };
    
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if comparator(&x[i], &x[i+mid_point]) == swap_condition {
            x.swap(i, i + mid_point);
        }
    }
}

// 3-3-7 単体テストを書く（数値のソート）
// このモジュールはcargo testを実行した時のみコンパイルされる
#[cfg(test)]
mod tests {
    // superは親モジュール（third）
    use super::{sort, sort_by};
    use crate::SortOrder::*;
    use crate::utils::{new_u32_vec, is_sorted};
    
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
    
    #[derive(Debug, PartialEq)]
    struct Student{
        // 構造体の定義ではフィールドの型を省略できない
        first_name: String,
        last_name: String,
        age: u8,
    }
    
    impl Student {
        fn new(first_name: &str, last_name: &str, age:u8) -> Self{
            // 構造体Studentを初期化して返す
            // SelfはStudent（impl対象）のこと
            Self { 
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                // フィールドと変数の名前が同じ時は、省略形で書ける
                age,
            }
        }
        
    }
    
    #[test]
    fn sort_students_by_age_ascending(){
        let taro = Student::new("Taro","Yamada",16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito",15);
        let ryosuke = Student::new("Ryosuke","Hayashi",17);
        
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];
        
        assert_eq!(
            sort_by(&mut x, &|a,b| a.age.cmp(&b.age)),
            Ok(())
        );
        
        assert_eq!(x, expected);
    }
    
    #[test]
    fn sort_students_by_name_ascending(){
        let taro = Student::new("Taro","Yamada",16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito",15);
        let ryosuke = Student::new("Ryosuke","Hayashi",17);
        
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        
        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];
        
        assert_eq!(
            sort_by(&mut x,
                &|a,b| a.last_name.cmp(&b.last_name)
                    .then_with(|| a.first_name.cmp(&b.first_name))
            ),  
            Ok(())
        );
        
        assert_eq!(x, expected);
    }
    
    #[test]
    fn sort_u32_large(){
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Ascending), Ok(()));
            assert!(is_sorted(&x, &Ascending));
        }
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Descending), Ok(()));
            assert!(is_sorted(&x, &Descending));
        }
    }
}