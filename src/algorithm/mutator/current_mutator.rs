use crate::algorithm::individual::Individual;
use crate::algorithm::mutator;
use crate::algorithm::mutator::Mutate;
use crate::algorithm::population::Population;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct CurrentMutator {
    pub(super) size: usize,
}

impl Mutate for CurrentMutator {
    fn mutate(&self, factor: f64, current: &Individual, population: &Population) -> Individual {
        mutator::difference(
            current,
            self.size,
            factor,
            population
                .iter()
                .filter(|x| x.as_ptr() != current.as_ptr())
                .collect(),
        )
    }

    fn required_population_size(&self) -> usize {
        self.size * 2 + 1
    }
}
