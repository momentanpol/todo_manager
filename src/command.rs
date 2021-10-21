use crate::todo;

#[derive(Debug)]
pub enum Command {
    Add(todo::Todo),
    Delete(todo::Todo),
    // ShowAll,
    // Update(Todo),
}

impl From<Vec<String>> for Command {
    fn from(args: Vec<String>) -> Command {
        // TODO: length of needed arguments differs for commands. The error handling should reflect that.
        if args.len() < 3 {
            panic!("Invalid input")
        }
        let cmd = &args[1];
        let tail = &args[2..];

        match cmd.to_lowercase().as_str() {
            "add" => {
                let until_idx = tail.iter().position(|c| c.contains("@"));
                let mut until = None;
                if let Some(idx) = until_idx {
                    until = Some(String::from(&tail[idx]));
                }
                // let kewords = tail.iter().position(|c| c.contains("#")).unwrap();
                let message = tail.join(" ");
                println!("{:?}", until);
                let todo = todo::TodoBuilder::new()
                    .message(message)
                    .until(until)
                    .build();
                Command::Add(todo)
            },
            "del" => {
                let id: usize = tail.join(" ").parse().unwrap();
                let todo = todo::TodoBuilder::new()
                    .id(id)
                    .build();
                Command::Delete(todo)
            },
            _ => panic!("Command not supported"),
        }
    }
}