// Good morning! Here's your coding interview problem for today.
// おはようございます！今日のコーディング面接問題です。
// This problem was asked by Airbnb.
// この問題はAirbnbに質問されました。
// Given a list of integers, write a function that returns the largest sum of non-adjacent numbers. Numbers can be 0 or negative.
// 整数のリストが与えられたとき，隣接しない数の最大和を返す関数を書きなさい．各数値には0や負の値を指定できます。
// For example, [2, 4, 6, 2, 5] should return 13, since we pick 2, 6, and 5. [5, 1, 1, 5] should return 10, since we pick 5 and 5.
// 例えば、[2, 4, 6, 2, 5]は13を返します。5, 1, 1, 5] は 10 を返します。
// Follow-up: Can you do this in O(N) time and constant space?
// フォローアップ：O(N)の時間と一定の空間でこれができますか？

use proconio::input;

#[allow(dead_code)]
pub fn main() {
    input! {
        n: i32,
        g: [i32; n]
    }
    println!("input: {:?}", g);
    // println!("output: {}", f(g));
}

// fn f(g: Vec<i32>) -> i32 {
//     // index: n+0 1 2 3 4 | 5 6 7 8 9
//     //    a)    x   x   x | max(c,d,e)
//     //    b)    x     x   | max(a,b,e)
//     //    c)      x   x   | max(a,b,e)
//     //    d)      x     x | max(c,d,e)
//     // ** e)        x   x | max(c,d,e)
//     let mut smax: i32 = 0;
//     smax
// }