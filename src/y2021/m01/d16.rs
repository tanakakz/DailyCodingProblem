// Good morning! Here's your coding interview problem for today.
// おはようございます！今日もお疲れ様でした。今日のコーデ面接問題です。
// This problem was asked by Twitter.
// この問題はTwitterで出題されました。
// Implement an autocomplete system. That is, given a query string s and a set of all possible query strings, return all strings in the set that have s as a prefix.
// オートコンプリートシステムを実装しなさい。つまり、クエリ文字列sと、可能なクエリ文字列の集合が与えられたとき、その集合の中でsを接頭辞として持つ文字列をすべて返す。
// For example, given the query string de and the set of strings [dog, deer, deal], return [deer, deal].
// 例えば、クエリ文字列 de と文字列 [dog, deer, deal] の集合が与えられた場合、[deer, deal] を返します。
// Hint: Try preprocessing the dictionary into a more efficient data structure to speed up queries.// ヒント: クエリを高速化するために、辞書をより効率的なデータ構造に前処理してみてください。

use proconio::input;

#[allow(dead_code)]
pub fn main() {
    // ex. type stdin:
    // de 3 dog deer deal
    input! {
        qs: String,
        n: usize,
        g: [String; n]
    }
    // ex. show stdout:
    // input: de ["dog", "deer", "deal"]
    // output: ["deer", "deal"]
    println!("input: {} {:?}", &qs, g);
    println!("output: {:?}", f(&qs, g));
}

fn f(qs: &str, g: Vec<String>) -> Vec<String> {
    return g.into_iter().filter(|x| x.starts_with(qs)).collect::<Vec<_>>();
}