use crate::heuristic::{HeuristicElem};

// Hill Climbing by iteration limit
pub fn hill_climbing(initial_state: Box<dyn HeuristicElem>, iteration_limit: u32) -> Box<dyn HeuristicElem> {
    let mut current_state = initial_state;
    let mut current_value = current_state.evaluate();

    for _ in 0..iteration_limit {
        let neighbours = current_state.get_neighbours();
        for neighbour in neighbours.iter() {
            let neighbour_value = neighbour.evaluate();
            if neighbour_value <= current_value {
                current_state = (*neighbour).dyn_clone();
                current_value = neighbour_value;
            }
        }
    }

    current_state
}

