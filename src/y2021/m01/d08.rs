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

#[allow(dead_code)]
pub fn main() {
    println!("at 2021/01/08\n");
    // let tree = node! {
    //     val: "root",
    //     left: node! {
    //         val: "left",
    //         left: node! {
    //             val: "left.left"
    //         }
    //     },
    //     right: node! {
    //         val: "right"
    //     },
    // };
    // let tree_left_left_val = Node::deserialize(tree.serialize()).left.unwrap().left.unwrap().val;
    // assert_eq!(tree_left_left_val.to_string(), "left.left");
}

// struct Node {
//     val: Box<str>,
//     left: Box<Option<Node>>,
//     right: Box<Option<Node>>,
// }
//
// impl Node {
//     fn new(val: &str, left: Node, right: Node) -> Node {
//         Node {
//             val: val.into(),
//             left: Box::new(Some(left)),
//             right: Box::new(Some(right)),
//         }
//     }
//     fn serialize(self) -> &'static str {
//         return "dummy";
//     }
//     fn deserialize(tree: &str) -> Node {
//         return Node::new( tree, Nil, nil );
//     }
// }
//
// #[macro_export]
// macro_rules! node {
//     ( val: $val:expr, left: $left:expr, right: $right:expr $(,)? ) => {
//         Node::new($val, $left, $right)
//     };
//     ( val: $val:expr, left: $left:expr $(,)? ) => {
//         node! { val: $val, left: $left, right: node!() }
//     };
//     ( val: $val:expr, right: $right:expr $(,)? ) => {
//         node! { val: $val, left: node!(), right: $right }
//     };
//     () => {
//         None
//     };
// }