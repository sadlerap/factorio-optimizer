use std::collections::HashMap;
use std::error::Error;

use crate::factorio::{Machine, Product, Recipe};
use good_lp::Expression;
use good_lp::{constraint, default_solver, variable, variables, Solution, SolverModel};
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;

/// Our model, which consists of the recipies, products, and machines we've
/// defined, as well as the necessary state to solve our model.
#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    recipies: Vec<Recipe>,
    products: Vec<Product>,
    machines: Vec<Machine>,
}

impl Model {
    /// Creates a new model.
    pub fn new(recipies: Vec<Recipe>, products: Vec<Product>, machines: Vec<Machine>) -> Self {
        Self {
            recipies,
            products,
            machines,
        }
    }
}

/// A solver for our model.
///
/// Our constants (invariant over the lifetime of the model) are the following:
/// - B_m -> production bonus of a machine
/// - P_rp -> how much of product p is produced in recipe r
/// - C_rp -> how much of product p is consumed in recipe r
///
/// Our variables are the following:
///
/// - M_mr -> how many machines of type m produce under recipe r
///
/// Our one required constraint is the following:
///
/// - sum(B_m M_mr P_rp for m in M for r in R) - sum(B_m M_mr C_rp for m in M for r in R) == 0 for p in P
///
#[derive(Debug)]
pub struct Solver {
    model: Model,
    production_constraints: HashMap<Product, f64>,
}

impl Solver {
    pub fn new(model: Model) -> Self {
        Self {
            model,
            production_constraints: HashMap::new(),
        }
    }

    pub fn add_production_constraint(&mut self, product: Product, amount_per_minute: f64) {
        if self.model.products.contains(&product) {
            self.production_constraints
                .insert(product, amount_per_minute);
        }
    }

    pub fn solve(&self) -> Result<f64, Box<dyn Error>> {
        let mut vars = variables! {};
        let machines = self
            .model
            .machines
            .iter()
            .cartesian_product(self.model.recipies.iter())
            .map(|(m, r)| {
                let v = vars.add(variable().integer().min(0).name(format!(
                    "machines-{}-{}",
                    m.name(),
                    r.name()
                )));
                ((m, r), v)
            })
            .collect::<HashMap<_, _>>();

        let overflow = self
            .model
            .products
            .iter()
            .map(|p| {
                let v = vars.add(variable().min(0).name(format!("{}-overflow", p.name())));
                (p, v)
            })
            .collect::<HashMap<_, _>>();

        let objective = machines
            .values()
            .copied()
            .fold(Expression::from_other_affine(0u8), |acc, x| acc + x);
        let mut problem = vars.minimise(&objective).using(default_solver);

        self.model.products.iter().for_each(|p| {
            let production_rate: Expression = self
                .model
                .recipies
                .iter()
                .map(|recipe| {
                    recipe.production_of(p).map_or_else(
                        || Expression::from_other_affine(0),
                        |rate| {
                            self.model
                                .machines
                                .iter()
                                .map(|machine| {
                                    machine.production_rate()
                                        * rate
                                        * (60.0 / recipe.production_time())
                                        * machines.get(&(machine, recipe)).cloned().unwrap()
                                })
                                .fold(Expression::from_other_affine(0), |acc, x| acc + x)
                        },
                    )
                })
                .sum();

            let consumption_rate: Expression = self
                .model
                .recipies
                .iter()
                .map(|recipe| {
                    recipe.usage_of(p).map_or_else(
                        || Expression::from_other_affine(0),
                        |rate| {
                            self.model
                                .machines
                                .iter()
                                .map(|machine| {
                                    machine.production_rate()
                                        * rate
                                        * (60.0 / recipe.production_time())
                                        * machines.get(&(machine, recipe)).cloned().unwrap()
                                })
                                .fold(Expression::from_other_affine(0), |acc, x| acc + x)
                        },
                    )
                })
                .sum();

            let extra = overflow.get(p).map_or_else(
                || Expression::from_other_affine(0),
                Expression::from_other_affine,
            );

            problem.add_constraint(constraint!(production_rate - consumption_rate == extra));
        });

        self.production_constraints.iter().for_each(|(p, v)| {
            let consumption_rate: Expression = self
                .model
                .recipies
                .iter()
                .map(|r| (r, r.usage_of(p)))
                .map(|(recipe, rate)| {
                    self.model
                        .machines
                        .iter()
                        .map(|machine| {
                            machine.production_rate()
                                * rate.unwrap_or(0.0)
                                * (60.0 / recipe.production_time())
                                * machines.get(&(machine, recipe)).cloned().unwrap()
                        })
                        .fold(Expression::from_other_affine(0), |acc, x| acc + x)
                })
                .sum();

            let extra = overflow.get(p).map_or_else(
                || Expression::from_other_affine(0),
                Expression::from_other_affine,
            );

            let needed_production = Expression::from_other_affine(*v);
            problem.add_constraint(constraint!(consumption_rate + extra >= needed_production));
        });

        Ok(problem.solve()?.eval(objective))
    }
}
