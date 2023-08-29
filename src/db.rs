use rusqlite::{params, Connection, Result};
use crate::models::ToDo; // ToDo structをインポート

// データの閲覧
pub fn console_view(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT task_num, task_check, task_event FROM ToDo")?;
    let task_iter = stmt.query_map([], |row| {
        //SQLiteはBoolen型が使えないのでここでにINTEGERをintに変換
        let task_check_int: i32 = row.get(1)?;
        Ok(ToDo::new(
            row.get(0)?,
            task_check_int != 0,
            row.get(2)?,
        ))
    })?;

    let mut has_data = false;  // フラグ変数

    for task_result in task_iter {
        has_data = true;
        
        match task_result {
            Ok(task) => {
                let task_num = match task.task_num() {
                    Some(num) => num.to_string(),
                    None => "None".to_string(),
                };

                let check_mark = if task.task_check() {
                    "☒"  // タスクが完了している場合
                } else {
                    "☐"  // タスクが未完了の場合
                };

                println!(
                    "{:^3}{:^3}{}",
                    check_mark,       // ☒ or ☐
                    task_num,         // タスク番号
                    task.task_event() // タスクのイベント
                );
            },
            Err(err) => println!("Error occurred: {:?}", err),
        }
    }

    if !has_data {  // データがなかった場合
        println!("No data");
    }

    Ok(())
}

//データの追加
pub fn task_add (conn: &Connection, task: &ToDo) -> rusqlite::Result<()> {
    //SQLiteはBoolen型が使えないのでここでintに変換
    let task_check_int = if task.task_check() { 1 } else { 0 };
    conn.execute(
        "INSERT INTO ToDo (task_check, task_event) VALUES (?1, ?2)",
        params![task_check_int, &task.task_event()],
    )?;
    Ok(())
}

//データ反転プログラム
pub fn toggle_task(conn: &Connection, choose_task_num: i32) -> Result<()>{
    conn.execute(
        "UPDATE ToDo SET task_check = NOT task_check WHERE task_num = ?1",
         params![choose_task_num],
        )?;
    Ok(())
}

//データの削除
pub fn task_remove (conn: &Connection, choose_task_num: i32) -> rusqlite::Result<()> {
    conn.execute(
        "DELETE FROM ToDo WHERE task_num = ?1",
        params![choose_task_num],
    )?;
    Ok(())
}