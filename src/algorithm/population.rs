// https://chatgpt.com/c/6708149d-d00c-8010-9c76-73eab9fa66ff
struct Individual;
struct Population(Vec<Individual>);

trait Evaluate {
    fn evaluate(&self) -> f64;
}

impl Population {
    fn evolve(self, mutator: Mutator, evaluator: &dyn Evaluate) -> Population {
        todo!()
    }
}

struct Mutator;

impl Mutator {
    fn mutate(individuals: &[Individual]) -> Individual {
        todo!()
    }
}

struct Crossover;

impl Crossover {
    fn crossover(parent: &Individual, mutant: Individual) -> Individual {
        todo!()
    }
}
