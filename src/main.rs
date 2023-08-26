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
mod cli;
use std::io::{self, Write};

use rusqlite::{Connection, Result};

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

    //ループ処理
    loop {
        println!("What would you like to do?");
        println!("1. Add task");
        println!("2. Delete task");
        println!("3. Show tasks");
        println!("4. Quit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                // タスクを追加
                println!("Adding task...");
                // ここで `conn` を使用してデータベースにタスクを追加する
                // CLI データの追加を入力する関数
                match cli::cli_add() {
                    Ok(Some(todo)) => {
                        // ToDo情報が正常に取得できたら、DBに保存などの処理を行う
                        match db::task_add(&conn, &todo) {
                            Ok(_) => println!("Task added successfully"),
                            Err(e) => println!("Failed to add task: {:?}", e),
                        }
                    }
                    Ok(None) => {
                        // ユーザーが"exit"を入力したか、何も入力しなかった場合の処理
                        println!("Exiting program.");
                        continue;  // これはループを終了するための仮定です。実際のコードに合わせて調整してください。
                    }
                    Err(e) => {
                        // エラーが発生した場合の処理
                        eprintln!("An error occurred: {:?}", e);
                    }
                }
            },
            2 => {
                // タスクを削除
                println!("Deleting task...");
                // ここで `conn` を使用してデータベースからタスクを削除する
            },
            3 => {
                // タスクを表示
                println!("Showing tasks...");
                // ここで `conn` を使用してデータベースのタスクを表示する
                // データの表示を処理する関数
                match db::console_view(&conn) {
                    Ok(_) => println!("Success!"),
                    Err(e) => println!("An error occurred: {:?}", e),
                }
            },
            4 => {
                println!("Quitting...");
                break;
            },
            _ => {
                println!("Invalid choice, please try again.");
            },
        }
    }  
    Ok(())
}

//Todo
// taskを完了させる処理がない
// CLIの表示非表示を処理する。crossterm と termion
