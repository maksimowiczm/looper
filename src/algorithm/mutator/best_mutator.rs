use crate::algorithm::individual::Individual;
use crate::algorithm::mutator;
use crate::algorithm::mutator::Mutate;
use crate::algorithm::population::Population;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct BestMutator {
    pub(super) size: usize,
}

impl Mutate for BestMutator {
    fn mutate(&self, factor: f64, _: &Individual, population: &Population) -> Individual {
        let best = population.best();

        mutator::difference(
            best,
            self.size,
            factor,
            population
                .iter()
                .filter(|x| x.as_ptr() != best.as_ptr())
                .collect(),
        )
    }

    fn required_population_size(&self) -> usize {
        self.size * 2 + 1
    }
}
