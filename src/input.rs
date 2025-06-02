use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::graph::{AdjacencyMatrix, Cost, INF};

fn parse_line(line: &str, expected_cols: usize) -> Result<Vec<Cost>, String> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.len() != expected_cols {
        return Err(format!("Input error: Expected {} cost values, found {}. Line: '{}'", expected_cols, parts.len(), line.trim()));
    }
    parts.iter().map(|s| {
            if s.eq_ignore_ascii_case("inf") {Ok(INF)} 
            else {s.parse::<Cost>().map_err(|_e| format!("Input error: Invalid cost value '{}'", s))}
        }).collect()
}


pub fn keyboard_input() -> Result<AdjacencyMatrix, String> {
    let mut input_buffer = String::new();

    print!("Enter the number of cities (N): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_buffer).map_err(|_e| "Input error: Failed to read number of cities".to_string()) ?;
    let n = input_buffer.trim().parse::<usize>().map_err(|_e| format!("Input error: Invalid number of cities '{}'", input_buffer.trim()))?;

    if n == 0 {return Ok(AdjacencyMatrix::from_vec(vec![]));}

    println!("Enter the adjacency matrix row by row ({} values per row, space-separated, use 'INF' for infinity):", n);

    let mut matrix_data: Vec<Vec<Cost>> = Vec::with_capacity(n);
    for i in 0..n {
        input_buffer.clear();
        print!("Row {} ({} values): ", i, n);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_buffer).map_err(|_e| format!("Input error: Failed to read matrix row {}", i))?;
        let row = parse_line(&input_buffer, n)?;
        matrix_data.push(row);
    }
    Ok(AdjacencyMatrix::from_vec(matrix_data))
}


pub fn file_input(filename: &str) -> Result<AdjacencyMatrix, String> {
    let path = Path::new(filename);
    let file = File::open(path).map_err(|_e| format!("Error: Could not open file '{}'", filename))?;

    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let n_line = lines.next().ok_or_else(|| "File error: File is empty or first line (number of cities) missing.".to_string())?
                      .map_err(|_e| "File error: Could not read the first line (number of cities).".to_string())?;

    let n = n_line.trim().parse::<usize>().map_err(|_e| format!("File error: Invalid number of cities '{}'", n_line.trim()))?;
    if n == 0 {return Ok(AdjacencyMatrix::from_vec(vec![]));}

    let mut matrix_data: Vec<Vec<Cost>> = Vec::with_capacity(n);
    for i in 0..n {
        let line_str = lines.next().ok_or_else(|| format!("File error: Expected {} matrix rows, but file ended after row {}.", n, i))? 
                            .map_err(|_e| format!("File error: Could not read matrix row {}.", i))?;
        
        let row = parse_line(&line_str, n)?;
        matrix_data.push(row);
    }
    Ok(AdjacencyMatrix::from_vec(matrix_data))
}
