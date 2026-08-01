// 所有権(ownership)の復習用練習コード。
// `cargo run --example ownership_exercises` で実行して動作を確認する。
// TODOを埋めて、コメントで指示された挙動になるようにしてください。

fn exercise_1_copy() {
    // TODO: i32型の変数 x に適当な数値を代入し、y に x を代入してください。
    // Copy型なので、この後 x と y の両方を println! で表示できるはずです。
    let x = 1;
    let y = x;
    println!("{}{}", x, y);
}

fn exercise_2_move() {
    // TODO: String::from(...) で s1 を作り、s2 に s1 を代入してください。
    // その後 s2 だけを表示してください(s1 は使わない)。
    //
    // 試しに println!("{}", s1); を追加してコンパイルしてみて、
    // どんなエラーメッセージが出るか確認してみましょう(確認したらコメントアウトに戻す)。
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); コンパイルエラーが起きる
}

fn exercise_3_clone() {
    // TODO: exercise_2_move と同様に s1 を作りますが、
    // 今度は s2 = s1.clone() を使って、s1 と s2 の両方を表示できるようにしてください。
    let s1 = String::from("yukiya");
    let s2 = s1.clone();
    println!("{}{}", s1, s2)
}

fn main() {
    exercise_1_copy();
    exercise_2_move();
    exercise_3_clone();
}
