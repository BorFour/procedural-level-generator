
use rsgenetic::pheno::Phenotype;

pub trait Level: Phenotype<i32> {
    fn generate_individual() -> Self;
    fn show(&self);
}
