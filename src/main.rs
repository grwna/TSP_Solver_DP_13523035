pub mod graph;
pub mod solver;
pub mod input;
pub mod output;
pub mod component;

use crate::component::{opt1, opt2, opt3, opt4, invalid, clear_screen};
use crate::graph::{AdjacencyMatrix};
use std::io::{self, Write};

fn title(){
    println!("\n========================================");
    println!("  Traveling Salesperson Problem Solver  ");
    println!("========================================");
    println!("              By 13523035               ");
    println!("                                        ");
    println!("                                        ");
    println!("               1. Start                  ");
    println!("               2. Exit                   ");
    println!("                                        ");
    println!("========================================");


}

fn main_menu(graph: &str){
    println!("\n========================================");
    println!("  Traveling Salesperson Problem Solver");
    println!("========================================");
    println!("Current Graph: [{}]", graph);
    println!("\nPlease choose an option:");
    println!("  1. Load Adjacency Matrix from File (.txt)");
    println!("  2. Enter Adjacency Matrix Manually");
    println!("  3. Solve TSP");
    println!("  4. View Current Graph");
    println!("  5. Exit");
    print!("Enter your choice (1-5): ");
    std::io::stdout().flush().unwrap();
}

fn main(){
    clear_screen();
    title();
    let mut start = String::new();
    io::stdin().read_line(&mut start).expect("Failed to read line");

    match start.trim().parse::<u32>() {
        Ok(1) => {}
        Ok(2) => {return;}
        _ => {println!("Invalid choice. Please enter a number between 1 or 2.");}
    }


    let mut current_graph: Option<AdjacencyMatrix> = None;

     loop {
        let menu_status = match &current_graph {
            Some(graph) => format!("Loaded, {} cities", graph.cities()),
            None => "Not Loaded".to_string(),
        };

        clear_screen();
        main_menu(&menu_status);
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().parse::<u32>() {
            Ok(1) => {
                opt1(&mut current_graph);
            }
            Ok(2) => {
                opt2(&mut current_graph);
            }
            Ok(3) => {
                opt3(&current_graph);
            }
            Ok(4) => {
                opt4(&current_graph);
            }
            Ok(5) => {
                println!("Exiting solver. Goodbye!");
                break;
            }
            _ => {
                invalid();
            }
        }
     }
}