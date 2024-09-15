use std::{
    io::{self, Write},
    process::exit,
};

use cfonts::{render, say, Options};
use colored::Colorize;

struct InputBuffer {
    buffer: String,
    buffer_length: usize,
    input_length: usize,
}

impl InputBuffer {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            buffer_length: 0,
            input_length: 0,
        }
    }
    fn read_input(&mut self) -> io::Result<()> {
        io::stdout().flush()?;
        // 清空之前的缓冲区
        self.buffer.clear();

        // 从标准输入读取一行
        let bytes_read = io::stdin().read_line(&mut self.buffer)?;

        // 检查读取结果
        if bytes_read <= 0 {
            eprintln!("Error reading input");
            std::process::exit(1);
        }

        self.input_length = bytes_read - 1;
        if self.buffer.ends_with('\n') {
            self.buffer.pop(); // remove trailing newline
        }
        if self.buffer.ends_with('\r') {
            self.buffer.pop(); // remove enter
        }
        self.buffer_length = self.buffer.len();

        Ok(())
    }

    // for dot command
    fn do_meta_command(&self) -> MetaCommandResult {
        if self.buffer == ".exit".to_string() {
            exit(0);
        } else {
            return MetaCommandResult::MetaCommandUnrecognizedCommand;
        }
    }
}

fn print_promt() {
    print!("{}", "🌌>> ".bright_purple());
}

fn tui() {
    // interface TUI
    let tui = render(Options {
        text: String::from("Rustdb"),
        font: cfonts::Fonts::Font3d,
        colors: vec![
            cfonts::Colors::Rgb(cfonts::Rgb::Val(147, 52, 136)),
            cfonts::Colors::WhiteBright,
        ],
        ..Options::default()
    });
    say(tui.options);
}

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedCommand,
}

fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    if input_buffer.buffer.get(0..6).unwrap() == "insert" {
        statement.type_t = StatementType::StatementInsert;
        return PrepareResult::PrepareSuccess;
    }
    if input_buffer.buffer == "select" {
        statement.type_t = StatementType::StatementSelect;
        return PrepareResult::PrepareSuccess;
    }

    return PrepareResult::PrepareUnrecognizedCommand;
}

enum StatementType {
    StatementInsert,
    StatementSelect,
}

struct Statement {
    type_t: StatementType,
}

fn execute_statement(statement: &Statement) {
    match statement.type_t {
        StatementType::StatementInsert => {
            println!("{}", "This is where we would do an insert".bright_green())
        }
        StatementType::StatementSelect => {
            println!("{}", "This is where we world do a select".bright_green())
        }
    }
}

fn main() {
    tui();

    let mut input_buffer = InputBuffer::new();
    loop {
        print_promt();
        input_buffer.read_input().unwrap();

        if input_buffer.buffer == ".exit".to_string() {
            exit(0);
        } else {
            println!("Unrecognized command {}.", input_buffer.buffer.red());
        }
    }
}
