use std::collections::HashMap;

use factorio_optimizer::{
    factorio::{Machine, Product, Recipe},
    solver::{Model, Solver},
};

#[test]
fn coal_production() {
    let machines = vec![Machine::new("Electric mining drill".to_owned(), 0.5)];
    let products = vec![Product::new("Coal".to_owned())];
    let recipies = vec![Recipe::new(
        "Coal mining".to_owned(),
        1.0,
        HashMap::new(),
        HashMap::from([(products[0].clone(), 1.0)]),
    )];

    let mut solver = Solver::new(Model::new(recipies, products, machines));
    solver.add_production_constraint(Product::new("Coal".to_owned()), 30.0);

    assert_eq!(solver.solve().unwrap(), 1.0);
}
