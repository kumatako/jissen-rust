use num_cpus;

// プロジェクトの名前はハイフンだけどここではアンダーバーを使う
use ch3_bitonic_sorter::SortOrder;
// 関数を別名で使う
use ch3_bitonic_sorter::third::sort as seq_sort;
use ch3_bitonic_sorter::forth::sort as par_sort;
use ch3_bitonic_sorter::utils::{is_sorted_ascending,new_u32_vec};

use std::{env,f64};
use std::str::FromStr;
use std::time::Instant;

fn main(){
    // 一つ目(0から開始)のコマンドライン引数を取得する
    if let Some(n) = env::args().nth(1){
        let bits = u32::from_str(&n).expect("error parsing argument");
        run_sorts(bits);
    }else{
        eprintln!(
            "Usage {} <number of elements in bits>",
            env::args().nth(0).unwrap()
        );
        std::process::exit(1);
    }   
}

fn run_sorts(bits: u32){
    let len = 2.0_f64.powi(bits as i32) as usize;
    
    println!(
        "sorting {} integers ({:.1} MB)",
        len,
        (len * std::mem::size_of::<u32>()) as f64 / 1024.0 / 1024.0
    );
    
    println!(
        "cpu info: {} physical cores, {} logical cores",
        num_cpus::get_physical(),
        num_cpus::get()
    );
    
    let seq_duration = timed_sort(&seq_sort, len, "seq_sort");
    let par_duration = timed_sort(&par_sort, len, "par_sort");
    
    println!("speed up: {:.2}x", seq_duration / par_duration);
}

fn timed_sort<F>(sorter: &F, len: usize, name: &str) -> f64
where
    F:Fn(&mut [u32], &SortOrder) -> Result<(),String>
{
    let mut x = new_u32_vec(len);
    
    let start = Instant::now();
    sorter(&mut x, &SortOrder::Ascending).expect("Faild to sort: ");
    let dur = start.elapsed();
    
    let nano_secs = dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64;
    println!(
        "{}: sorted {} integers in {} seconds",
        name,
        len,
        nano_secs / 1e9
    );
    
    assert!(is_sorted_ascending(&x));
    
    nano_secs
}