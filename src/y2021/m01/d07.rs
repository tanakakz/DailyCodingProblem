// Good morning! Here's your coding interview problem for today.
// おはようございます！今日もお疲れ様でした。今日のコーディング面接問題です。
// This problem was asked by Uber.
// この問題はUberからの質問です。
// Given an array of integers, return a new array such that each element at index i of the new array is the product of all the numbers in the original array except the one at i.
// 整数の配列が与えられたとき、新しい配列のインデックス i にある各要素が、i にあるものを除く元の配列のすべての数値の積となるような新しい配列を返してください。
// For example, if our input was [1, 2, 3, 4, 5], the expected output would be [120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected output would be [2, 3, 6].
// 例えば、入力が [1, 2, 3, 4, 5] の場合、期待される出力は [120, 60, 40, 30, 24] となります。入力が [3, 2, 1] の場合、期待される出力は [2, 3, 6] となります。
// Follow-up: what if you can't use division?
// フォローアップ：割り算が使えない場合は？

use proconio::input;
// https://docs.rs/proconio/0.4.1/proconio/

#[allow(dead_code)]
pub fn main() {
    println!("at 2021/01/07\n");
    println!("type n:int  g:[int, n]");
    println!("input> ");
    input! {
        n: usize,   // ex. 5
        g: [i32; n] // ex. 1 2 3 4 5
    }
    println!("{:?}", f(n, g)); // ex. [120, 60, 40, 30, 24]
}

fn f(n: usize, mut g: Vec<i32>) -> Vec<i32> {
    let mut ret: Vec<i32> = (&mut g).clone();
    for i in 0..n {
        ret[i] = 1;
        for j in 0..n {
            if i == j { continue; }
            ret[i] *= g[j];
        }
    }
    return ret;
}
