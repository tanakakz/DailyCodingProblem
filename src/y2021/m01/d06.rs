// Good morning! Here's your coding interview problem for today.
// おはようございます！今日もお疲れ様でした。今日のコーディング面接問題です。
// This problem was recently asked by Google.
// この問題は最近Googleに質問された問題です。
// Given a list of numbers and a number k, return whether any two numbers from the list add up to k.
// 数のリストと数kが与えられたとき、リストの中から2つの数がkに足し算されるかどうかを返しなさい。
// For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.
// 例えば、[10, 15, 3, 7]と17のkが与えられた場合、10 + 7は17なので、trueを返します。
// Bonus: Can you do this in one pass?
// ボーナス: 1回のパスでこれができますか？

use proconio::input;
// https://docs.rs/proconio/0.4.1/proconio/

#[allow(dead_code)]
pub fn main() {
    println!("at 2021/01/06\n");
    println!("type n:int  g:[int, n]  k:int");
    println!("input> ");
    input! {
        n: usize,      // ex. 4
        g: [i32; n], // ex. 10 15 3 7
        k: i32         // ex. 17
    }
    println!("{}", f(n, g, k)); // ex. true
}

fn f(n: usize, mut g: Vec<i32>, k: i32) -> bool {
    // 要件：g[i]+g[j]=k, i<jとなる2要素がgivenに含まれるかどうか

    // givenの要素数nが最小必要要素数2に満たない場合、要件を満たせないと判断する
    if n < 2 { return false; }

    // givenの要素を昇順に並び替える
    g.sort();

    // 2要素合計の最小値がkより大きい場合、要件を満たせないと判断する
    if k < g[0]+g[1] { return false; }

    // 2要素合計がkと等しくなるものが見つからないと判断したした場合中断する
    if false { return false; }

    return true;
}
