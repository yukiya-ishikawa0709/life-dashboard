// ライフタイムの復習用練習コード。
// `cargo run --example lifetime_exercises` で実行して動作を確認する。
// TODOを埋めて、コメントで指示された挙動になるようにしてください。

// 以下は「ダングリング参照」になってしまう壊れた例。まずはコメントを外して
// ビルドし、コンパイルエラー(戻り値の生存期間に関するエラー)を確認してみてください。
// 確認したら、また // でコメントアウトしておいてください
// (呼ばれていなくても関数の中身にエラーがあるとビルド全体が失敗するのは前回学んだ通りです)。
//
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String{
    let s = String::from("unko");
    s
}

fn exercise_1_return_owned() {
    // TODO: dangle() のように内部で作った文字列を「参照」ではなく「所有権ごと」
    // 返す関数 no_dangle を定義してください。戻り値の型は &String ではなく String にします。
    // fn no_dangle() -> String { ... }
    //
    // 定義できたら no_dangle() を呼び出して結果を表示してください。
    let result = no_dangle();
    println!("{}", result)
}

// TODO: 2つの文字列スライス(&str)を受け取り、長い方を返す関数 longest を、
// ライフタイム注釈 'a を使って定義してください。
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {x} else {y}
}

fn exercise_2_longest() {
    let s1 = String::from("hello");
    let s2 = String::from("world!!");
    // TODO: longest(&s1, &s2) を呼び出して結果を表示してください。
    // s2 の方が長いので "world!!" が表示されるはずです。
    let result = longest(&s1, &s2);
    println!("{}", result)
}

fn main() {
    exercise_1_return_owned();
    exercise_2_longest();
}
