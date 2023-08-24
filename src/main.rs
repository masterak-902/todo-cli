/// # Rust Todo アプリケーションのサンプル作成
/// ### ドキュメントの作成方法について
/// "///" 使ってコメントアウトすることで、ドキュメントを作成することができる。
/// ### crateの追加・削除
/// '''
/// cargo add ~~ で追加
/// cargo rm ~~ で削除
/// '''
/// Todo で使用するcrateの一覧
/// ・rusqlite SQLiteを使うためのクレート

/// ## Todoアプリケーションのデータ構造
/// ### Taskcheck
/// bool型で、このTaskの完了未完了を確認します。
/// ### task_num
/// このタスクの検索用名です。重複は許されません一意の識別子です。
/// ### task_event
/// 

// main.rs

mod models; // models.rsをインポート
mod db; // db.rsをインポート

use rusqlite::{Connection, Result};
use crate::models::ToDo; // ToDo structをインポート

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE ToDo (
            task_num    INTEGER PRIMARY KEY,
            task_check  INTEGER,
            task_event  TEXT NOT NULL
        )",
        (),
    )?;
    
    let me = ToDo::new(0, false, "example".to_string());

    match db::task_add(&conn, &me) {
        Ok(_) => println!("Task added successfully"),
        Err(e) => println!("Failed to add task: {:?}", e),
    }
    
    match db::console_view(&conn) {
        Ok(_) => println!("Success!"),
        Err(e) => println!("An error occurred: {:?}", e),
    }
    Ok(())
}
//データの追加
/*  Todo 仮データを作成する。
    //  SeaCreatureのデータはスタックに入ります。
    #[allow(dead_code)]
    let event_info = ToDo {
        // String構造体もスタックに入りますが、
        // ヒープに入るデータの参照アドレスが一つ入ります。
        task_check: false,
        task_num: 1,
        task_event: String::from("詳細を記入する。"),

        //構造体からデータを取り出す際は、構造体のフィールドは演算子"."で取り出すことができます。
        // "今日のイベント名は、{}で詳細は{}です。", event_info.name, event_info.event 
        // Result : 今日のイベント名は、タイトルで詳細は詳細を記入する。です。
    };
*/
//データの更新

