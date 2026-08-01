// 借用(borrowing)の復習用練習コード。
// `cargo run --example borrowing_exercises` で実行して動作を確認する。
// TODOを埋めて、コメントで指示された挙動になるようにしてください。

// TODO: String への不変参照(&String)を受け取り、その長さ(usize)を返す関数
// calculate_length を定義してください。
// fn calculate_length(...) -> usize { ... }
fn calculate_length(s: &String) -> usize{
    s.len()
}

fn exercise_1_immutable_borrow() {
    let s1 = String::from("hello");
    // TODO: calculate_length(&s1) を呼び出して len に代入してください。
    // その後 s1 と len の両方を println! で表示してください(s1はムーブされていないはず)。
    let len = calculate_length(&s1);
    println!("{}{}", s1, len);
}

// TODO: String への可変参照(&mut String)を受け取り、" world" を追記する関数
// append_world を定義してください(戻り値なし)。
// fn append_world(...) { ... }
fn append_world(s: &mut String){
    s.push_str(" world");
}

fn exercise_2_mutable_borrow() {
    let mut s = String::from("hello");
    // TODO: append_world(&mut s) を呼び出し、その後 s を表示してください。
    // "hello world" と表示されるはずです。
    append_world(&mut s);
    println!("{}", s); //hello world
}

// 以下は「不変参照と可変参照が同時に存在してコンパイルエラーになる」例。
// 関数の中身もコメントアウトしないと、呼び出されていなくてもビルド自体が失敗する
// (Rustは呼ばれるかどうかに関係なく関数の中身を型・借用チェックするため)。
// fn exercise_3_conflict() {
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &mut s; // ここで E0502: cannot borrow `s` as mutable because it is also borrowed as immutable
//     println!("{}, {}", r1, r2);
// }

fn exercise_4_fix_conflict() {
    let mut s = String::from("hello");
    let r1 = &s;
    println!("{}", r1); //r1の使用を終わる
    let r2 = &mut s; // TODO: まずはこのままビルドし、コンパイルエラーを確認してください。
    println!("{}", r2);

    // 確認できたら、r1 の最後の使用(println!の中)より前で r2 を作らないように
    // 修正してみてください。ヒント: r1 を使い終わってから r2 を作る、つまり
    // r1 を使う println! を r2 の可変参照を作るより前に済ませてしまう、という順序に変えます。
}

fn main() {
    exercise_1_immutable_borrow();
    exercise_2_mutable_borrow();
    exercise_4_fix_conflict();
}
