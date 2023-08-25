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
// mod cli;

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

    /*
    // CLI データの追加を入力する関数
    match cli::cli_add() {
        Ok(todo) => {
            // ToDo情報が正常に取得できたら、DBに保存などの処理を行う
            println!("Successfully add a ToDo: {:?}", todo);
        }
        Err(e) => {
            // エラーが発生した場合の処理
            eprintln!("An error occurred: {:?}", e);
        }
    }*/

    let me = ToDo::new(Some(0), false, "example".to_string());
    // データ追加を処理する関数
    match db::task_add(&conn, &me) {
        Ok(_) => println!("Task added successfully"),
        Err(e) => println!("Failed to add task: {:?}", e),
    }
    
    // データの表示を処理する関数
    match db::console_view(&conn) {
        Ok(_) => println!("Success!"),
        Err(e) => println!("An error occurred: {:?}", e),
    }
    Ok(())
}

