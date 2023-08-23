/// # Rust Todo アプリケーションのサンプル作成
/// ### ドキュメントの作成方法について
/// "///" 使ってコメントアウトすることで、ドキュメントを作成することができる。

#[allow(dead_code)] 
struct ToDo {
    task_check: bool,
    name: String,
    event: String,
}

fn main() {

    
    // Todo 仮データを作成する。
    //  SeaCreatureのデータはスタックに入ります。
    #[allow(dead_code)]
    let event_info = ToDo {
        // String構造体もスタックに入りますが、
        // ヒープに入るデータの参照アドレスが一つ入ります。
        task_check: false,
        name: String::from("タイトル"),
        event: String::from("詳細を記入する。"),

        //構造体からデータを取り出す際は、構造体のフィールドは演算子"."で取り出すことができます。
        // "今日のイベント名は、{}で詳細は{}です。", event_info.name, event_info.event 
        // Result : 今日のイベント名は、タイトルで詳細は詳細を記入する。です。
    };

}
