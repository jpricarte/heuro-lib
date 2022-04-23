pub trait HeuristicElem {
    fn evaluate(&self) -> f64;
    fn get_neighbours(&self) -> Vec<Box<dyn HeuristicElem>>;
    fn dyn_clone(&self) -> Box<dyn HeuristicElem>;
}
