# heuro-lib
Generic implementation of classical heuristics, in Rust.
(non-classical heuristics are welcome).

## Implemented Heuristics
- Hill climbing

## How to use
Implement `HeuristicElem` trait in a struct with this functions:
```Rust
fn evaluate(&self) -> f64; // return a float based in this element

fn get_neighbors(&self) -> Vec<Box<dyn HeuristicElem>>; // return a vector of neighbors of this element
fn dyn_clone(&self) -> Box<dyn HeuristicElem>; // return a Box containing the element itselfs
```

## TODO
- [ ] implement generic stop condition for any heuristic
- [ ] implement Simulated Annealing in local_search.rs
- [ ] implement TABU Search in local_search.rs
- [ ] implement genetic.rs and genetic heuristics
- [ ] create Python interface for heuro-lib
- [ ] create Wasm interface for heuro-lib (why not?)

And other things.
