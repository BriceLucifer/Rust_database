use std::{
    io::{self, Write},
    process::exit,
};

use cfonts::{render, say, Options};
use colored::Colorize;

// InputBuffer structure
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
        self.buffer.clear();

        let bytes_read = io::stdin().read_line(&mut self.buffer)?;

        if bytes_read <= 0 {
            eprintln!("Error reading input");
            exit(1);
        }

        self.input_length = bytes_read - 1;
        if self.buffer.ends_with('\n') {
            self.buffer.pop();
        }
        if self.buffer.ends_with('\r') {
            self.buffer.pop();
        }
        self.buffer_length = self.buffer.len();

        Ok(())
    }

    fn do_meta_command(&self) -> MetaCommandResult {
        if self.buffer == ".exit" {
            println!("{}", "👋 Bye! See you soon!".bright_yellow());
            exit(0);
        } else {
            MetaCommandResult::MetaCommandUnrecognizedCommand
        }
    }
}

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

#[derive(Debug, Clone)]
pub struct Table {
    num_rows: u32,
    pages: Vec<Option<Vec<u8>>>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            num_rows: 0,
            pages: vec![None],
        }
    }

    fn row_slot(&mut self, row_num: u32) -> &mut [u8] {
        let page_num = (row_num / 100) as usize;
        let row_offset = (row_num % 100) as usize * 48;

        if page_num >= self.pages.len() {
            self.pages.push(None);
        }

        let page = self.pages[page_num].get_or_insert_with(|| vec![0; 4096]);
        &mut page[row_offset..row_offset + 48]
    }

    pub fn insert(&mut self, row: &Row) {
        if self.num_rows >= 10000 {
            eprintln!("Error: Table full.");
            return;
        }
        let row_data = serialize_row(row);
        let slot = self.row_slot(self.num_rows);
        slot.copy_from_slice(&row_data);
        self.num_rows += 1;
    }

    pub fn print(&self) {
        let mut rows = Vec::new();
        for i in 0..self.num_rows {
            let row = deserialize_row(self.clone().row_slot(i as u32));
            rows.push(row);
        }

        // Print the table with borders and headers
        let header = format!("{:<5} {:<20} {:<30}", "ID", "Username", "Email");
        let separator = "-".repeat(header.len());
        println!("{}", header);
        println!("{}", separator);
        for row in rows {
            println!("{:<5} {:<20} {:<30}", row.id, row.username, row.email);
        }
    }
}

// Serialize a row into a fixed-size byte array
fn serialize_row(row: &Row) -> Vec<u8> {
    let mut data = vec![0; 48]; // Fixed size for each row
    let id_bytes = row.id.to_le_bytes();
    data[0..4].copy_from_slice(&id_bytes);

    // Make sure to truncate strings if they are too long
    let username_bytes = &row.username.as_bytes()[..row.username.len().min(20)];
    let email_bytes = &row.email.as_bytes()[..row.email.len().min(30)];

    data[4..4 + username_bytes.len()].copy_from_slice(username_bytes);
    data[4 + 20..4 + 20 + email_bytes.len()].copy_from_slice(email_bytes);

    data
}

// Deserialize a row from a fixed-size byte array
fn deserialize_row(data: &[u8]) -> Row {
    let id = i32::from_le_bytes(data[0..4].try_into().unwrap());
    let username = String::from_utf8(data[4..24].to_vec())
        .unwrap()
        .trim_end_matches('\0')
        .to_string();
    let email = String::from_utf8(data[24..48].to_vec())
        .unwrap()
        .trim_end_matches('\0')
        .to_string();
    Row {
        id,
        username,
        email,
    }
}

// Prepare execute for next operation
enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedCommand,
    PrepareSyntaxError,
}

fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    if input_buffer.buffer.starts_with("insert") {
        statement.type_t = StatementType::StatementInsert;
        let args: Vec<&str> = input_buffer.buffer.split_whitespace().collect();
        if args.len() < 4 {
            return PrepareResult::PrepareSyntaxError;
        }
        statement.row_to_insert.id = args[1].parse().unwrap();
        statement.row_to_insert.username = args[2].to_string();
        statement.row_to_insert.email = args[3].to_string();
        PrepareResult::PrepareSuccess
    } else if input_buffer.buffer == "select" {
        statement.type_t = StatementType::StatementSelect;
        PrepareResult::PrepareSuccess
    } else {
        PrepareResult::PrepareUnrecognizedCommand
    }
}

enum StatementType {
    StatementInsert,
    StatementSelect,
}

struct Statement {
    type_t: StatementType,
    row_to_insert: Row,
}

impl Statement {
    fn execute_statement(&self, table: &mut Table) {
        match self.type_t {
            StatementType::StatementInsert => {
                table.insert(&self.row_to_insert);
                println!("{}", "Executed insert.".cyan());
            }
            StatementType::StatementSelect => {
                table.print();
            }
        }
    }
}

pub struct Row {
    id: i32,
    username: String,
    email: String,
}

fn main() {
    tui();

    let mut input_buffer = InputBuffer::new();
    let mut table = Table::new();
    loop {
        print_prompt();
        input_buffer.read_input().unwrap();

        if input_buffer.buffer.starts_with('.') {
            match input_buffer.do_meta_command() {
                MetaCommandResult::MetaCommandSuccess => {}
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!(
                        "{} {}",
                        "Unrecognized command:".yellow(),
                        input_buffer.buffer.bright_red()
                    );
                }
            }
        } else {
            let mut statement = Statement {
                type_t: StatementType::StatementSelect,
                row_to_insert: Row {
                    id: 0,
                    username: String::new(),
                    email: String::new(),
                },
            };

            match prepare_statement(&input_buffer, &mut statement) {
                PrepareResult::PrepareSuccess => {
                    statement.execute_statement(&mut table);
                }
                PrepareResult::PrepareUnrecognizedCommand => {
                    println!(
                        "{} {}",
                        "Unrecognized command:".yellow(),
                        input_buffer.buffer.bright_red()
                    );
                }
                PrepareResult::PrepareSyntaxError => {
                    println!(
                        "{}",
                        "Syntax Error! Could not parse statement.".bright_red()
                    );
                }
            }
        }
    }
}

fn print_prompt() {
    print!("{} ", "👉 Enter command:".bold().blue());
    io::stdout().flush().unwrap();
}

fn tui() {
    let tui = render(Options {
        text: String::from("DataBase"),
        font: cfonts::Fonts::Font3d,
        colors: vec![
            cfonts::Colors::Rgb(cfonts::Rgb::Val(211, 84, 0)),
            cfonts::Colors::WhiteBright,
            cfonts::Colors::Rgb(cfonts::Rgb::Val(211, 84, 0)),
        ],
        ..Options::default()
    });
    say(tui.options);
    println!("\n{}", "============================".bright_green());
    println!("{}", "  Your fancy command-line".italic().yellow());
    println!("{}", "============================".bright_green());
    println!();
}
