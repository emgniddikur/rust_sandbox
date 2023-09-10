//! # concurrency
//!
//! ## 並行プログラミングと並列プログラミング
//!
//! 並行プログラミング⋯複数のプログラムが独立して実行すること
//!
//! 並列プログラミング⋯複数のプログラムが同時に実行すること
//!
//! ## スレッド
//!
//! OSは同時に複数のプロセス（実行中のプログラム）を管理する
//!
//! スレッド⋯プログラムの実行単位。
//! プログラムを実行するとスレッドが1つ起動する。
//!
//! スレッド自体も同時に走らせることができる。コードの走る順番に関して保証はない
//!
//! 新規スレッドはメインスレッドが終了したら停止する
//!
//! 言語がOSのAPIを呼び出してスレッドを生成するモデルを1:1モデルと呼ぶ
//!
//! 言語が提供するスレッドはグリーンスレッドで、それを異なる数のOSスレッドで実行するモデルをM:Nモデルと呼ぶ
//!
//! Rustにとって最も重要な各モデルの代償は、ランタイム（全てのバイナリに含まれるコード）のサポート
//!
//! M:Nのグリーンスレッドモデルは、スレッドを管理するのにより大きな言語ランタイムを必要とするので、Rustの標準ライブラリは1:1スレッドのみを提供している
//!
//! M:Nスレッドのクレートを使用することで、オーバーヘッドと引き換えに、どのスレッドをいつ走らせるかの制御や、より低コストの文脈切り替えなどができるようになる

mod channel;
mod mutex;
mod spawn_join;

fn main() {
    channel::main();
    mutex::main();
}
