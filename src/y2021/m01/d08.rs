// Good morning! Here's your coding interview problem for today.
// おはようございます！今日もお疲れ様でした。今日のコーディング面接問題です。
// This problem was asked by Google.
// この問題はGoogleさんからの質問でした。
// Given the root to a binary tree, implement serialize(root), which serializes the tree into a string, and deserialize(s), which deserializes the string back into the tree.
// バイナリツリーへのルートを与えられたとき、ツリーを文字列にシリアライズするserialize(root)と、文字列をツリーに戻すdeserialize(s)を実装しなさい。
// For example, given the following Node class
// 例えば、次のようなノードクラスが与えられたとします。
//
// class Node:
// def __init__(self, val, left=None, right=None):
//   self.val = val
//   self.left = left
//   self.right = right
//
// The following test should pass:
// 以下のテストはパスするはずです。
//
// node = Node('root', Node('left', Node('left.left')), Node('right'))
// assert deserialize(serialize(node)).left.left.val == 'left.left'

// use proconio::input;
// https://docs.rs/proconio/0.4.1/proconio/

#[allow(dead_code)]
pub fn main() {
    println!("at 2021/01/08\n");
    println!("input> ");
}

struct Node {
    // dummy
}

#[allow(unused)]
fn serialize(root: Node) -> string {
    return "dummy";
}

fn deserialize(tree: string) -> Node {
    return Node!();
}