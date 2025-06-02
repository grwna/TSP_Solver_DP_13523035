# TSP_Solver_DP
> Traveling Salesman Problem solver using Dynamic Programming

<br/>
<div align="center" id="contributor">
 <strong>
  <h3> Author </h3>
  <table align="center">
    <tr align="center">
      <td>NIM</td>
      <td>Name</td>
      <td>GitHub</td>
    </tr>
    <tr align="center">
      <td>13523035</td>
      <td>M. Rayhan Farrukh</td>
      <td><a href="https://github.com/grwna">@grwna</a></td>
    </tr>
  </table>
 </strong>
 <br>
 <br>
</div>

---

<div align="center">

### Tech Stack and Language 

<img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" style="width:120px; height:auto; border-radius: 10px;">
</div>

## 📝 Project Description
This program is a simple implementation of the Traveling Salesman Problem (TSP) solver using Dynamic Programming with bitmasking approach. TSP is a classic problem in computer science and operations research that aims to find the shortest possible route a salesperson can take to visit a set of cities, visiting each city exactly once, and returning to the origin city.

The Dynamic Programming approach used here is known as the Held-Karp algorithm, has a time complexity of $O(N^2 \cdot 2^N)$, where $N$ is the number of cities. This makes it suitable for a relatively small number of cities.

## ⚙️ How It Works
The program operates through the following main steps:

1.  **Graph Representation**: Distances or costs between cities are represented using an adjacency matrix.
2.  **Input**: The user can provide the adjacency matrix in two ways:
    * **Keyboard Input**: Manually enter the number of cities and then the cost values for each row of the matrix.
    * **File Input**: Read the matrix from a text file. The file should be formatted as follows: the first line contains the number of cities ($N$), followed by $N$ lines, each containing $N$ cost values (space-separated). The string "INF" can be used to represent non-existent direct paths (infinity).
3.  **Solver Algorithm (Dynamic Programming with Bitmask)**:
    * **DP State**: `dp[mask][i]` stores the minimum cost to visit all cities included in the `mask` (a bitwise representation of the set of visited cities), starting from a designated `start_node` (city 0 by default) and ending at city `i`.
    * **Base Case**: `dp[1 << start_node][start_node] = 0`, which means the cost to visit only the starting city and end there is 0.
    * **Transition**: The core recurrence relation is `dp[mask][i] = min(dp[mask ^ (1 << i)][j] + cost[j][i])` for all cities `j` present in the `mask` (excluding `i`). This means to find the cost of reaching city `i` after  visiting the set of cities in `mask`, we consider all possible previous cities `j`.
    * **Parent Tracking**: To reconstruct the path, a `parent[mask][i]` table stores all the previous city `j` for the optimal path to city `i` with the given `mask`.
4.  **Path Reconstruction**: After the DP table is filled and the minimum total tour cost is found, the optimal path is reconstructed by backtracking using the `parent` table.
5.  **Output**: The program displays:
    * The minimum tour cost.
    * The execution time of the solver.
    * The optimal tour path (sequence of cities).
    * The program also allows users to save the result to a file.

## 🚀 How to Use

1.  **Prerequisites**:
    * Rust and Cargo: install [here](https://www.rust-lang.org/tools/install).

2.  **Compilation**:
    * Clone the repository:
        ```
        
        ```
    * Navigate to the project's root directory in your terminal.
    * Compile the program using Cargo:
        ```bash
        cargo build
        ```
        For an optimized release build:
        ```bash
        cargo build --release
        ```

3.  **Running the Program**:
    * Run the compiled executable. If you used `cargo build --release`, the executable will be in `target/release/`. Otherwise, it's in `target/debug/`.
        ```bash
        # For release build
        ./target/release/your_project_name
        # For debug build
        ./target/debug/your_project_name
        ```
        (Replace `your_project_name` with the actual name of your executable, usually the name of your crate/project).
    * Alternatively, you can run directly using Cargo:
        ```bash
        cargo run
        ```

4.  **Interacting with the Program**:
    * Upon running, a menu will be displayed:
        ```text
        ========================================
          Traveling Salesperson Problem Solver
        ========================================
        Current Graph: [Not Loaded / N cities]

        Please choose an option:
          1. Load Adjacency Matrix from File (.txt)
          2. Enter Adjacency Matrix Manually
          3. Solve TSP (requires graph to be loaded)
          4. View Current Graph (if loaded)
          5. Save Current Graph to File (if loaded)
          6. Exit

        Enter your choice (1-6): _
        ```
    * **Option 1 (Load from File)**:
        * Prompts for a filename (e.g., `matrix.txt`).
        * The file should be formatted as:
            ```
            N
            cost_0_0 cost_0_1 ... cost_0_(N-1)
            cost_1_0 cost_1_1 ... cost_1_(N-1)
            ...
            cost_(N-1)_0 ... cost_(N-1)_(N-1)
            ```
            Where `N` is the number of cities. Use "INF" for infinite costs.
    * **Option 2 (Enter Manually)**:
        * Prompts for the number of cities.
        * Then prompts for each row of the adjacency matrix.
    * **Option 3 (Solve TSP)**:
        * Solves the TSP for the currently loaded graph.
        * Displays the solution (cost, time, path, path details).
        * Asks if you want to save the solution to a file.
    * **Option 4 (View Graph)**:
        * Displays the currently loaded adjacency matrix on the console.
    * **Option 5 (Save Graph to File)**:
        * Asks for a filename and saves the current adjacency matrix to that file in the same format used for input.
    * **Option 6 (Exit)**: Exits the program.

## 📸 Screenshot(s) of Input/Output

*Replace this section with your screenshot(s). You can use Markdown image syntax:*
```markdown
