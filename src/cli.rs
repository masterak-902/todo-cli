use std::io::{self, Write};
use crate::models::ToDo; 

pub fn cli_add() -> io::Result<ToDo>{
    print!("Enter the task event: ");
    io::stdout().flush()?;  // ?を使ってエラーを上位に伝播
    
    let mut task_event = String::new();
    io::stdin().read_line(&mut task_event)?;  // ?を使ってエラーを上位に伝播
    let task_event = task_event.trim().to_string();  // 改行文字を取り除く
    
    // すべてが成功した場合は、Okで包んだToDoオブジェクトを返す
    Ok(ToDo::new(None, false, task_event))
}