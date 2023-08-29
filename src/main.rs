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

//モジュールインポート
mod models;
mod db;
mod cli;

//デフォルトクレート
use std::io::{self, Write};

//外部クレート
use crossterm::{self, terminal::Clear, ExecutableCommand, cursor};
use rusqlite::{Connection, Result};

///`Box<dyn std::error::Error>` 
/// Rustでよく使われるエラーハンドリングのパターンの一つです。
/// これは "boxed dynamic trait object" であり、`std::error::Error` トレイトを実装する任意の型を表します。
///- `Box`: ヒープに格納される値を指すスマートポインタ。
///- `dyn`: "dynamic dispatch"を使用することを示すキーワード。これにより、コンパイル時ではなく実行時にメソッドが呼び出されます。
///- `std::error::Error`: エラーハンドリング用の基本的なトレイト。`std::fmt::Debug` と `std::fmt::Display` トレイトを要求します。
///
///`Box<dyn std::error::Error>` は、異なるエラー型を同一の型として扱いたい場面でよく使用されます。これにより、異なるエラー型を返す可能性がある関数でも、一つの統一されたエラー型を返すようにすることができます。
///
///例えば、以下のような関数があるとします。
///
///```rust
///// この関数は io::Error か ParseIntError を返す可能性がある
///fn do_something() -> Result<(), Box<dyn std::error::Error>> {
///    let f = File::open("some_file.txt")?;  // io::Error を返す可能性がある
///    let parsed_num: i32 = "10".parse()?;  // ParseIntError を返す可能性がある
///    // ...（何らかの処理）
///    Ok(())
///}
///```
///この関数は、`io::Error` を返す可能性もあれば、`ParseIntError` を返す可能性もあります。
///それぞれのエラー型に対して `?` 演算子を使用すると、それぞれのエラー型を `Box<dyn std::error::Error>` に自動的に変換してくれます。
/// これにより、関数は単一の `Result` 型を返すことができます。
///このような動的なエラー処理は便利ですが、実行時のオーバーヘッドがある場合もあります。
///そのため、パフォーマンスが重要な場面では他のエラーハンドリングの手法が選ばれることもあります。

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE ToDo (
            task_num    INTEGER PRIMARY KEY,
            task_check  INTEGER,
            task_event  TEXT NOT NULL
        )",
        (),
    )?;

    io::stdout().execute(Clear(crossterm::terminal::ClearType::All))?;
    //ループ処理
    loop {
        io::stdout().execute(cursor::MoveDown(3))?;

        println!("What would you like to do?");
        println!("1. Add task");
        println!("2. Delete task");
        println!("3. Show tasks");
        println!("4. Task Complete");
        println!("5. Quit");

        print!(" : ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

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

                match cli::choose_task() {
                    Ok(task_num) =>{
                        match db::task_remove(&conn, task_num) {
                            Ok(_) => println!("Task removed successfully"),
                        Err(e) => println!("Failed to remove task: {}", e),
                        }

                    }
                    Err(_) => println!("Invalid input"),
                }
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
                println!("Toggle tasks...");

                match cli::choose_task() {
                    Ok(task_num) =>{
                        match db::toggle_task(&conn, task_num) {
                            Ok(_) => println!("Task {} toggled", task_num),
                        Err(e) => println!("Failed to delete task: {}", e),
                        }

                    }
                    Err(_) => println!("Invalid input"),
                }
            },
            5 => {
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
// Delete機能の追加
