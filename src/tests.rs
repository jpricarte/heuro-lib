#[cfg(test)]
use crate::local_search::hill_climbing;
    use crate::heuristic::HeuristicElem;

    #[derive(Clone)]
    pub struct Bar {
        x: f64,
    }


    impl HeuristicElem for Bar {
        fn evaluate(&self) -> f64 {
            self.x
        }

        fn get_neighbors(&self) -> Vec<Box<dyn HeuristicElem>>  {
            vec!{Box::new(Bar{x: 0.0}),Box::new(Bar{x: 1000.0})}
        }

        fn dyn_clone(&self) -> Box<dyn HeuristicElem> {
            Box::new((*self).clone())
        }

    }
    #[test]
    fn hill_climbing_test() {
        let foo = Box::new(Bar{x: 5.0});
        let result = hill_climbing(foo, 1);
        assert_eq!((*result).evaluate(), 0.0);
    }