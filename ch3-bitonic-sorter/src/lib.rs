pub mod first;
pub mod second;
pub mod third;
pub mod utils;

// ソートの方向を表す型を列挙型として定義する
pub enum SortOrder{
    Ascending, //昇順
    Descending, //降順
}