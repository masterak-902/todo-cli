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

    for task_result in task_iter {
        match task_result {
            Ok(task) => println!(
                "Found task: task_num = {}, task_check = {}, task_event = {}",
                task.task_num(),
                task.task_check(),
                task.task_event()
            ),
            Err(err) => println!("Error occurred: {:?}", err),
        }
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