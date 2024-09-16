# Rust Database CLI Application

This document provides an overview of a simple command-line database application implemented in Rust. The application allows users to perform basic operations on a table, such as inserting and selecting rows.

## Code Overview

### Dependencies

- `cfonts`: For rendering styled ASCII text in the terminal.
- `colored`: For adding colors to terminal output.

### Structure

The application is organized into several key components:

#### 1. `InputBuffer` Structure

The `InputBuffer` structure handles user input. It reads input from the terminal, processes it, and determines if it matches any meta commands (e.g., `.exit`).

```rust
struct InputBuffer {
    buffer: String,
    buffer_length: usize,
    input_length: usize,
}

impl InputBuffer {
    fn new() -> Self { ... }
    fn read_input(&mut self) -> io::Result<()> { ... }
    fn do_meta_command(&self) -> MetaCommandResult { ... }
}
```
#### 2. `Table` Structure
The Table structure represents a table that stores rows. It supports inserting new rows and printing the table's contents.

```rust
#[derive(Debug, Clone)]
pub struct Table {
    num_rows: u32,
    pages: Vec<Option<Vec<u8>>>,
}

impl Table {
    pub fn new() -> Self { ... }
    fn row_slot(&mut self, row_num: u32) -> &mut [u8] { ... }
    pub fn insert(&mut self, row: &Row) { ... }
    pub fn print(&self) { ... }
}
```
#### 3. Row Structure
The Row structure represents a single row in the table. It includes an id, username, and email.

```rust
pub struct Row {
    id: i32,
    username: String,
    email: String,
}
```
#### 4. Serialization and Deserialization
Functions to serialize and deserialize rows to and from byte arrays are provided.

```rust
fn serialize_row(row: &Row) -> Vec<u8> { ... }
fn deserialize_row(data: &[u8]) -> Row { ... }
```
#### 5. Statement Preparation
The prepare_statement function parses user input and prepares the appropriate statement.

```rust
enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedCommand,
    PrepareSyntaxError,
}

fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult { ... }
```

#### 6. Statement Execution
The Statement structure represents a SQL statement and supports executing either insert or select operations.

```rust
enum StatementType {
    StatementInsert,
    StatementSelect,
}

struct Statement {
    type_t: StatementType,
    row_to_insert: Row,
}

impl Statement {
    fn execute_statement(&self, table: &mut Table) { ... }
}
```
## 7. Main Function
The main function sets up the CLI, handles user input, and processes commands.

```rust
fn main() { ... }
```

#### 8. Utility Functions
print_prompt: Prints the command prompt.
tui: Displays styled ASCII text in the terminal.
```rust
fn print_prompt() { ... }
fn tui() { ... }
```
## Usage
Start the Application: Run the Rust application.
```bash
git clone https://github.com/BriceLucifer/Rust_database.git   
cd Rust_database && cargo run
```
### Enter Commands:
```bash 
Insert: insert <id> <username> <email>
Select: select
Exit: .exit
```

```bash 
👉 Enter command: insert 1 helloworld hello@example.com
Executed insert.

👉 Enter command: select
ID    Username             Email
-----------------------------------------------
1     helloworld           hello@example.com
```
### License
This project is licensed under the `GNU` License.
