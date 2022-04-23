use crate::heuristic::{HeuristicElem};

// Hill Climbing by iteration limit
pub fn hill_climbing(initial_state: Box<dyn HeuristicElem>, iteration_limit: u32) -> Box<dyn HeuristicElem> {
    let mut current_state = initial_state;
    let mut current_value = current_state.evaluate();

    for _ in 0..iteration_limit {
        let neighbors = current_state.get_neighbors();
        for neighbor in neighbors.iter() {
            let neighbor_value = neighbor.evaluate();
            if neighbor_value <= current_value {
                current_state = (*neighbor).dyn_clone();
                current_value = neighbor_value;
            }
        }
    }

    current_state
}

