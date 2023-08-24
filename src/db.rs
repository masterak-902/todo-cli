// db.rs

use rusqlite::{Connection, Result};
use crate::models::ToDo; // ToDo structをインポート

pub fn console_view(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT task_num, task_check, task_event FROM ToDo")?;
    let task_iter = stmt.query_map([], |row| {
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