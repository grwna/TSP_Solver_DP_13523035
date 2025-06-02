use crate::graph::{AdjacencyMatrix, Cost, INF, TourSolution};


pub fn solve(graph: &AdjacencyMatrix) -> Option<TourSolution> {
    let n = graph.cities();

    if n == 0 {
        return Some(TourSolution {cost: 0, path_indices: vec![], path_details: vec![]});
    } 
    if n == 1 {
        return Some(TourSolution {cost: 0, path_indices: vec![0, 0], path_details: vec![]});
    }

    let start_node: usize = 0;
    let num_masks = 1 << n; // 2^n possible subsets

    // dp[mask][last_city_index]
    let mut dp = vec![vec![INF; n]; num_masks];
    let mut parent = vec![vec![0; n]; num_masks];

    // Base Case
    dp[1 << start_node][start_node] = 0;

    for mask in 1..num_masks {
        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                // set of cities visited before 'i'
                let prev_mask = mask ^ (1 << i);
                if prev_mask == 0 {
                    continue;
                }

                for j in 0..n {
                    // is j in prev mask?
                    if (prev_mask & (1 << j)) != 0 {
                        if dp[prev_mask][j] != INF && graph.cost(j, i) != INF {
                            let new_cost = dp[prev_mask][j] + graph.cost(j, i);
                            if new_cost < dp[mask][i] {
                                dp[mask][i] = new_cost;
                                parent[mask][i] = j;
                            }
                        }
                    }
                }
            }
        }
    }

    // Calculate the final tour cost
    let final_mask = (1 << n) - 1;
    let mut min_tour_cost = INF;
    let mut last_city = 0;

    for i in 0..n {
        if dp[final_mask][i] != INF && graph.cost(i, start_node) != INF {
            let tour_cost = dp[final_mask][i] + graph.cost(i, start_node);
            if tour_cost < min_tour_cost {
                min_tour_cost = tour_cost;
                last_city = i;
            }
        }
    }

    if min_tour_cost == INF {
        return None
    } else {
        return Some(reconstruct_path(n, start_node, final_mask, last_city, min_tour_cost, &parent, graph))
    }
}

fn reconstruct_path( n: usize, start_node: usize, final_mask: usize, last_city: usize, min_tour_cost: Cost, parent: &Vec<Vec<usize>>, graph: &AdjacencyMatrix) -> TourSolution {
    let mut path_indices = Vec::with_capacity(n + 1);
    let mut path_details = Vec::with_capacity(n);

    let mut current_city = last_city;
    let mut current_mask = final_mask;

    if n > 0 {
        for _ in 0..(n - 1) {
            path_indices.push(current_city);
            let prev_city = parent[current_mask][current_city];
            current_mask ^= 1 << current_city;
            current_city = prev_city;
        }
        path_indices.push(start_node);
        path_indices.reverse();
    }


    let mut final_path = path_indices.clone();

    if n > 1 {
        for k in 0..(n - 1) {
            let from = path_indices[k];
            let to = path_indices[k + 1];
            path_details.push((from, to, graph.cost(from, to)));
        }
        path_details.push((last_city, start_node, graph.cost(last_city, start_node)));
        final_path.push(start_node);
    } else if n == 1 {
        final_path = vec![0,0];
    }

    return TourSolution {cost: min_tour_cost, path_indices: final_path, path_details: path_details}
}