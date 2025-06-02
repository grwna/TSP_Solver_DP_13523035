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

## 📝 Program Structure
```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── src
│   ├── component.rs
│   ├── graph.rs
│   ├── input.rs
│   ├── main.rs
│   ├── output.rs
│   └── solver.rs
└── test
    ├── input1.txt
    ├── input2.txt
    ├── input3.txt
    ├── result1.txt
    └── result2.txt

3 directories, 15 files
```


## 🚀 How to Use

1.  **Prerequisites**:
    * Rust and Cargo: install [here](https://www.rust-lang.org/tools/install).

2.  **Compilation**:
    * Clone the repository:
        ```
        git clone https://github.com/grwna/TSP_Solver_DP_13523035.git
                                or
        git clone git@github.com:grwna/TSP_Solver_DP_13523035.git
        ```
    * Navigate to the project's root directory.
    * Compile the program using Cargo:
        ```bash
        cargo build
        ```
3.  **Running the Program**:
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
          5. Exit

        Enter your choice (1-5): _
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
    * **Option 5 (Exit)**: Exits the program.

## 📸 Input/Output Examples
### Input
1. 
```markdown
4
0 10 15 20
10 0 35 25
15 35 0 30
20 25 30 0
```

2.
```
4
0 10 INF 12
10 0 5 INF
INF 5 0 8
12 INF 8 0
```

3.
```
5
0  3 20 12  7
4  0  5 18 10
18 6  0  4  9
10 15 3  0  8
8 11  7  9  0
```

### Output
1. 
<img src="https://github.com/user-attachments/assets/dee33917-bd58-4c26-9b53-086db5c6c6c1" width="400" alt="Image 1">

2. 
<img src="https://github.com/user-attachments/assets/13b6c8d8-4d6f-487a-9799-6bc7043343e4" width="400" alt="Image 2">

3. 
<img src="https://github.com/user-attachments/assets/7ab358ad-4756-44ab-acbb-5047d8a84a96" width="400" alt="Image 3">