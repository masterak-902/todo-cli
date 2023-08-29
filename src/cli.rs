use std::io::{self, Write};
use crate::models::ToDo; 

pub fn cli_add() -> io::Result<Option<ToDo>>{
    print!("Enter the task event: ");
    io::stdout().flush()?;  // ?を使ってエラーを上位に伝播
    
    let mut task_event = String::new();
    io::stdin().read_line(&mut task_event)?;  // ?を使ってエラーを上位に伝播
    let task_event = task_event.trim().to_string();  // 改行文字を取り除く

    if task_event.is_empty() || task_event.to_lowercase() == "exit" {
        return Ok(None);  // ユーザーが何も入力しなかった、または"exit"と入力した場合
    }
    
    // すべてが成功した場合は、Okで包んだToDoオブジェクトを返す
    Ok(Some(ToDo::new(None, false, task_event)))
}

//データを取得して返すプログラム、数字以外の入力はエラーを返す。
pub fn choose_task() -> io::Result<i32> {
    loop {
        print!("Enter the task number(or type 'exit' to quit): ");
        io::stdout().flush()?;

        let mut task_num = String::new();
        io::stdin().read_line(&mut task_num)?;

        let trimmed_input = task_num.trim();

        if trimmed_input.to_lowercase() == "exit" {
            // exitの処理をここで行う
            // 例えば、Errを返してループを抜ける
            return Err(io::Error::new(io::ErrorKind::Other, "User exited"));
        }

        match trimmed_input.parse::<i32>() {
            Ok(num) => return Ok(num),
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}
