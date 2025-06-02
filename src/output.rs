use std::time::Duration;
use crate::graph::{TourSolution, Cost, INF, AdjacencyMatrix};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn display_solution(solution: &TourSolution, duration: Duration) {
    println!("\n--- Result ---");
    println!("Minimum tour cost:      {}", solution.cost as i32);
    println!("Execution time:         {:.6} ms", duration.as_secs_f64() * 1000.0);

    if solution.path_indices.is_empty() && solution.cost == 0 {
        println!("Optimal tour:           (No cities)");
        return;
    }
    if solution.path_indices == vec![0,0] && solution.cost == 0 && solution.path_details.is_empty() {
        println!("Optimal tour:           0 -> 0 (Only one city)");
        return;
    }

    print!("Optimal tour:           ");
    for (i, city_idx) in solution.path_indices.iter().enumerate() {
        if i > 0 {
            print!(" -> ");
        }
        print!("{}", city_idx);
    }
    println!();

    if !solution.path_details.is_empty() {
        println!("\nPath details:");
        let mut calculated_total_check: Cost = 0;
        for (from, to, cost_segment) in &solution.path_details {
            println!("  City {} -> City {}: {}", from, to, *cost_segment as i32);
            calculated_total_check += *cost_segment;
        }
        println!("  ------------------------------");
        println!("  Calculated total:   {}", calculated_total_check as i32);
    }
}


pub fn solution_to_file(solution: &TourSolution, duration: Duration, filename: &str ) -> io::Result<()> {
    let path = Path::new(filename);
    let mut file = File::create(path)?;

    writeln!(file, "--- Result ---")?;
    writeln!(file, "Minimum tour cost:      {}", solution.cost as i32)?;
    writeln!(file, "Execution time:         {:.3} ms", duration.as_secs_f64() * 1000.0)?;

    if solution.path_indices.is_empty() && solution.cost == 0 {
        writeln!(file, "Optimal tour:           (No cities)")?;
        return Ok(());
    }
    if solution.path_indices == vec![0,0] && solution.cost == 0 && solution.path_details.is_empty() {
        writeln!(file, "Optimal tour:           0 -> 0 (Only one city)")?;
        return Ok(());
    }

    let mut tour_path_str = String::from("Optimal tour:           ");
    for (i, city_idx) in solution.path_indices.iter().enumerate() {
        if i > 0 {
            tour_path_str.push_str(" -> ");
        }
        tour_path_str.push_str(&city_idx.to_string());
    }
    writeln!(file, "{}", tour_path_str)?;

    if !solution.path_details.is_empty() {
        writeln!(file, "\nPath details:")?;
        let mut calculated_total_check: Cost = 0;
        for (from, to, cost_segment) in &solution.path_details {
            writeln!(file, "  City {} -> City {}: {}", from, to, *cost_segment as i32)?;
            calculated_total_check += *cost_segment;
        }
        writeln!(file, "  ------------------------------")?;
        writeln!(file, "  Calculated total:   {}", calculated_total_check as i32)?;
    }
    Ok(())
}


pub fn display_matrix(graph: &AdjacencyMatrix) {
    println!("\n--- Adjacency Matrix ({} cities) ---", graph.cities());
    if graph.cities() == 0 {
        println!("(Graph is empty)");
        return;
    }
    print!("      ");
    for j in 0..graph.cities() {
        print!("{:>5} ", j);
    }
    println!("\n       ------------------------------------");

    for i in 0..graph.cities() {
        print!("Row {:<2}| ", i);
        for j in 0..graph.cities() {
            let cost = graph.cost(i, j);
            if cost == INF {print!("{:>5} ", "INF");} 
            else {print!("{:>5} ", cost);}
        }
        println!();
    }
}

