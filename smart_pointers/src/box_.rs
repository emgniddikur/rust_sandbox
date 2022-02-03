//! # Box
//!
//! 間接参照とヒープメモリ確保を行う
//!
//! データをヒープ上に格納する以外はオーバーヘッドはない
//!
//! ## 用途
//!
//! - コンパイル時にはサイズのわからない型を、サイズを要求するコンテキストで使う場合
//! - データをコピーせず、多くのデータの所有権を移したい場合
//! - 特定のトレイトを実装する型の値を所有したい場合（トレイトオブジェクト）

fn fn1() {
    let n = Box::new(0);
    println!("{}", n);
}

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn fn2() {
    let _list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
}

pub fn main() {
    fn1();

    fn2();
}
