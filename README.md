# jissen-rust
実践Rust入門（技術評論社）を読んで書いたコードです。

## メモ
rustc 1.68.0 (2c8cc3432 2023-03-06)
### 2-2-1 パッケージの作成
git管理されているディレクトリの下でcargo new --bin helloしたら、helloのパッケージの中に.gitは作られなかった。 

--binオプションでパッケージを作る：ビルドすると実行可能バイナリファイルができる。

--libオプションでパッケージを作る：ビルドするとrlibというライブラリファイルができる。

### 2-2-7 プログラムの内容
println!マクロの詳細についてのリンクが切れている。

### 2-3-4 Rust RLS拡張機能のインストール
拡張機能のRust(rust-lang.rust)は非推奨になっていた。代わりにrust-analyserを入れてみた。

### 2-4 RPN計算機プログラムとデバッガによる実行
RPN（Reverse Polish Notation、逆ポーランド記法）

「XX変数をYYの値に束縛する」という表現が、まだあまり腑に落ちない。

「値(YY)に名前(XX)をつける」と言われればわかった。

「XXという名前はYYの値を表すためだけに使う」といってみると「束縛」らしくなる。

クロージャ？　→ 3-5-4

トレイト境界？　→ 8-1-2

ライフタイム？ → 7章

### 3-3 第一段階：初歩的な実装

借用？　→ 第7章

`unimplimented!()`マクロ
- https://doc.rust-lang.org/std/macro.unimplemented.html
- コンパイルは通る。実行するとpanicが起きる。
- まだ中身を実装していないが、コンパイルして型チェックしたいときに使う。

### 3-4-3 大小比較可能な型に限定する
半順序？全順序？

- （ざっくり）半順序は比較できない組がある（例：浮動小数点型f64の2.0とNaN）
- ソートするなら比較できない組み合わせがあると困る　→ 全順序がいる

error[E0308]は`rustc --explain E0308`で説明してくれる。

### 3-5 第3段階：クロージャでソート順をカスタマイズ
トレイト境界をfn xxx<T: Ord>って書くのと、where節で書くのは何が違うの？
- 3-4では、Tは全順序の型しか受け付けたくなかったから<T: Ord>と書いた
- 3-5でクロージャを使う。
  - Tが全順序の型でなかったらクロージャ(`&|a,b| a.cmp(b)`)を書いた時点でコンパイルエラーが出るから、`<T: Ord>`が不要で`where F: ...`が必要？

### 3-7 仕上げ：ベンチマークプログラム
3-5までの逐次処理のソートと3-6の並列処理のソートを比較する。
```
sorting 67108864 integers (256.0 MB)
cpu info: 10 physical cores, 10 logical cores
seq_sort: sorted 67108864 integers in 23.574600584 seconds
par_sort: sorted 67108864 integers in 2.944962375 seconds
speed up: 8.01x
```