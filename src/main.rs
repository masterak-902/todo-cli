struct ToDo {
    task_check: bool,
    name: String,
    event: String,
}

fn main() {
    // スタティックメソッドでStringインスタンスを作成する。
    let s = String::from("Hello world!");
    // インスタンスを使ってメソッド呼び出す。
    println!("{} is {} characters long.", s, s.len());
}
