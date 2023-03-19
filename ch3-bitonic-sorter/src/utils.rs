use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

use crate::SortOrder;

pub fn new_u32_vec(n: usize) -> Vec<u32>{
    // RNG Random Number Generatorを初期化する
    let mut rng = Pcg64Mcg::from_seed([0;16]);
    
    /* 
    let mut v = Vec::with_capacity(n);
    
    for _ in 0..n{
        v.push(rng.sample(&Standard));
    }
    
    v
    */
    // rng.sample_iter()は乱数を無限に生成するイテレータを返す
    // take(n)は元のイテレータから最初のn要素だけを取り出すイテレータを返す
    // collectはイテレータの値をベクタやハッシュマップのようなコレクションに収納する
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool{
    // windows(2)はスライスから２要素ずつ順に取り出して新しいイテレータを返す
    // allはイテレータから値を取り出してクロージャに渡す
    // -- クロージャがtrueを返している間は次の値に進む
    // -- クロージャがfalseを返すとallはfalseを返して終わり
    // -- クロージャが全部trueを返すとallはtrueを返す
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool{
    x.windows(2).all(|pair| pair[0] >= pair[1])
}

// 一つの関数にまとめると
pub fn is_sorted<T: Ord>(x: &[T], order: &SortOrder) -> bool{
    match order {
        SortOrder::Ascending => x.windows(2).all(|pair| pair[0] <= pair[1]),
        SortOrder::Descending => x.windows(2).all(|pair| pair[0] >= pair[1]),
    }
}
    