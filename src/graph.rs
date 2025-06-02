pub type Cost = i32;
pub const INF: Cost = i32::MAX / 2;

#[derive(Debug, Clone)]
pub struct AdjacencyMatrix {
    matrix: Vec<Vec<Cost>>,
    cities: usize,
}

impl AdjacencyMatrix {
    pub fn from_vec(costs: Vec<Vec<Cost>>) -> Self {
        let cities = costs.len();
        AdjacencyMatrix {
            matrix: costs,
            cities,
        }
    }

    #[inline]
    pub fn cities(&self) -> usize {
        self.cities
    }

    #[inline]
    pub fn cost(&self, from_city: usize, to_city: usize) -> Cost {
        self.matrix[from_city][to_city]
    }
}

pub struct TourSolution {
    pub cost: Cost,
    pub path_indices: Vec<usize>, 
    pub path_details: Vec<(usize, usize, Cost)>,
}
