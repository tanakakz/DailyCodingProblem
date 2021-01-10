// Good morning! Here's your coding interview problem for today.
// おはようございます！今日のコーディング面接問題です。
// This problem was asked by Jane Street.
// この問題はジェーン・ストリートさんが出題しました。
// cons(a, b) constructs a pair, and car(pair) and cdr(pair) returns the first and last element of that pair. For example, car(cons(3, 4)) returns 3, and cdr(cons(3, 4)) returns 4.
// cons(a, b)はペアを作成し、car(pair)とcdr(pair)はそのペアの最初と最後の要素を返します。例えば、car(cons(3, 4))は3を返し、cdr(cons(3, 4))は4を返します。
// Given this implementation of cons:
// このconsの実装を考えると
//
// def cons(a, b):
//     def pair(f):
//         return f(a, b)
//     return pair
//
// Implement car and cdr.
// carとcdrを実装してください。
