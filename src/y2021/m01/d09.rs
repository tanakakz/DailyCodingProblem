// Good morning! Here's your coding interview problem for today.
// おはようございます！今日のコーディング面接問題です。
// This problem was asked by Stripe.
// この問題はStripeさんが質問したものです。
// Given an array of integers, find the first missing positive integer in linear time and constant space. In other words, find the lowest positive integer that does not exist in the array. The array can contain duplicates and negative numbers as well.
// 整数の配列が与えられたとき，線形時間と一定空間において，最初に欠けている正の整数を求めよ．言い換えれば，配列に存在しない最下位の正の整数を求めます．配列には重複や負の数も含まれます．
// For example, the input [3, 4, -1, 1] should give 2. The input [1, 2, 0] should give 3.
// 例えば、入力 [3, 4, -1, 1] は 2 を与えます。入力 [1, 2, 0] は 3 となります。
// You can modify the input array in-place.
// 入力配列をその場で変更することができます。

use proconio::input;
// https://docs.rs/proconio/0.4.1/proconio/

use itertools::Itertools;

#[allow(dead_code)]
pub fn main() {
    println!("at 2021/01/06\n");
    println!("type n:int  g:[int, n]  k:int");
    println!("input> ");
    input! {
        n: usize,      // ex. 4
        g: [i32; n], // ex. 3 4 -1 1
    }
    println!("{}", f(g)); // ex. 2
}

fn f(g: Vec<i32>) -> i32 {
    let mut ret: i32 = 1;
    for elm in g.into_iter().filter(|&v| v > 0).sorted().collect::<Vec<_>>() {
        // println!("{}, {}", ret, elm);
        if ret != elm { break; }
        ret+=1;
    }
    return ret;
}
