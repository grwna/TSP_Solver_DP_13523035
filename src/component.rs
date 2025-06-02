
use crate::graph::{AdjacencyMatrix, TourSolution};
use crate::input::{keyboard_input, file_input};
use crate::solver::solve;
use crate::output::{display_matrix, display_solution, solution_to_file};
use std::io::{self, Write};
use std::time::{Instant, Duration};

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}

fn press_enter(){
    print!("\nPress Enter to return to the menu...");
    io::stdout().flush().unwrap();
    let mut sampah = String::new();
    io::stdin().read_line(&mut sampah).expect("Failed to read line");
}

pub fn opt1(graph: &mut Option<AdjacencyMatrix>) {
    println!("Option 1: Load from File selected.");
    print!("Enter filename: ");
    io::stdout().flush().unwrap();
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("Failed to read filename");
    match file_input(filename.trim()) {
        Ok(graph_data) => {
            println!("Graph loaded successfully.");
            *graph = Some(graph_data);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    press_enter();
}

pub fn opt2(graph: &mut Option<AdjacencyMatrix>){
    println!("Option 2: Enter Manually selected.");
    match keyboard_input() {
        Ok(graph_data) => {
            println!("Graph entered successfully.");
            *graph = Some(graph_data);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    press_enter();
}

pub fn opt3(graph_: &Option<AdjacencyMatrix>){
    println!("Option 3: Solve TSP selected.");
    match &graph_ {
        Some(graph) => {
            if graph.cities() > 0 {
                println!("Solving TSP...");
                let start_time = Instant::now();
                match solve(graph) {
                    Some(solution) => {
                        let duration = start_time.elapsed();
                        display_solution(&solution, duration);
                        save_to_file_prompt(&solution, duration);
                    }
                    None => println!("No solution found."),
                }
            } else {
                println!("Graph is empty. Cannot solve.");
            }
        }
        None => println!("No graph loaded. Please load a graph first.")
    }
    press_enter();
}

pub fn opt4(graph: &Option<AdjacencyMatrix>){
    println!("Option 4: View Current Graph selected.");
    match &graph {
        Some(graph_) => display_matrix(graph_),
        None => println!("No graph loaded. Please load a graph first."),
    }
    press_enter();
}

pub fn invalid(){
    println!("Invalid choice. Please enter a number between 1 and 5.");
    press_enter();
}

fn save_to_file_prompt(solution: &TourSolution, duration: Duration) {
    let mut choice = String::new();

    loop {
        print!("Do you want to save this solution to a file? (y/n): ");
        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                let mut filename_input = String::new();
                print!("Enter the filename (e.g., solution_results.txt): ");
                io::stdout().flush().unwrap();
                filename_input.clear();
                io::stdin().read_line(&mut filename_input).expect("Failed to read filename");
                
                let filename = filename_input.trim();
                if filename.is_empty() {
                    println!("Filename cannot be empty. Save operation cancelled.");
                    return;
                }

                match solution_to_file(solution, duration, filename) {
                    Ok(()) => {println!("Solution successfully saved to '{}'.", filename);}
                    Err(e) => {eprintln!("Error saving solution to file '{}': {}", filename, e);}
                }
                return;
            }
            "n" | "no" => {
                println!("Solution will not be saved to a file.");
                return;
            }
            _ => {
                println!("Invalid choice. Please enter 'y' (yes) or 'n' (no).");
            }
        }
    }
}