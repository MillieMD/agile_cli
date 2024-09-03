use clap::{Parser, Subcommand};
use std::fs;
use std::option::Option;
use std::path::Path;

mod board;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    #[command(about = "List all boards")]
    List,

    #[command()]
    Select { board_name: String },

    #[command(about = "Display the details of a card, on the selected board")]
    View { card_name: String },

    #[command(about = "Create a new board, column, or card")]
    New {
        #[clap(subcommand)]
        sub: CreateObjectType,
    },

    #[command(about = "Block a card")]
    Block { card_name: String },

    #[command(about = "Delete a board, column or card")]
    Delete {
        #[clap(subcommand)]
        sub: DeleteObjectType,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum CreateObjectType {
    Board {
        board_name: String,
    },
    Column {
        col_name: String,
        location: String,
        #[arg(long, short)]
        wip: Option<usize>,
        default: Option<bool>,
    },
    Card {
        card_name: String,
        location: Option<String>,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum DeleteObjectType {
    Board { board_name: String },
    Column { col_name: String },
    Card { card_name: String },
}

fn main() {
    let args = Args::parse();

    let mut boards: Vec<board::Board> = load("temp.txt");
    let mut current: board::Board;

    match args.cmd {
        Command::List => view_boards(&boards),
        Command::Select { board_name } => match find_board(board_name, boards) {
            Some(board) => {
                current = board;
                println!("Current Board: {}", current.name());
            }
            None => println!("Board not found"),
        },
        Command::View { card_name } => println!("Showing details for {}", card_name),
        Command::New { sub, .. } => {
            match sub {

                CreateObjectType::Board { board_name } => {
                    println!("Creating new board '{}'", board_name);
                    new_board(board_name, &mut boards);
                    save(&boards, "temp.txt")
                }

                CreateObjectType::Column {
                    col_name,
                    location,
                    wip,
                    default,
                } => println!(
                    "new column name {} location {} wip limit {:?} defualt? {:?}",
                    col_name, location, wip, default
                ),
                CreateObjectType::Card {
                    card_name,
                    location,
                } => {
                    
                    let target_col : board::Column;

                    if location.is_none() {
                        
                    }else{
                        if find_column(location.unwrap(), current).is_some(){
                            target_col = find_column(location.unwrap(), current).unwrap() // Bad i know
                        }
                    }

                },
            }
        },

        Command::Block { card_name } => println!("blocking {}", card_name),
        Command::Delete { sub } => println!("deleting a {:?}", sub),
    }
}

fn load(location: &str) -> Vec<board::Board> {
    if !Path::new(location).exists() {
        let boards: Vec<board::Board> = vec![];
        return boards;
    }

    let data = fs::read_to_string(location).expect("Could not read temp.txt");
    serde_json::from_str(&data).unwrap()
}

fn save(boards: &Vec<board::Board>, location: &str) {
    let data = serde_json::to_string(boards).expect("Unable to parse board into JSON");

    fs::write(location, data).unwrap();
}

fn view_boards(boards: &Vec<board::Board>) {
    for board in boards {
        println!("{}", board.name());
    }
}

fn find_board(board_name: String, boards: Vec<board::Board>) -> Option<board::Board> {
    for board in boards {
        if *board.name() == board_name {
            return Some(board);
        }
    }

    return None;
}

fn find_column(col_ref : String, board : board::Board) -> Option<board::Column>{
    return Some(board::Column::new("placeholder return"));
}

fn new_board(name: String, boards: &mut Vec<board::Board>) {
    boards.push(board::Board::new(name.as_str()));
}

// new col, new card

fn delete_board() {}

// delete col, delete card

// edit col, edit card
