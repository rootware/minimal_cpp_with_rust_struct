use ffi::*;
//#[cxx::bridge(namespace = "org::environment")]

use cxx::CxxVector;

#[cxx::bridge]
mod ffi {

    extern "Rust" {
        type Environment;
        fn new_environment() -> Box<Environment>;
        fn print_acc(&self)->f64;
        fn weighted_sum_vector(&self, vec: &CxxVector<f64>)->f64;
    }


}

fn new_environment () -> Box<Environment> {
    Box::new(Environment{ weight: 2.0})
}

struct Environment {
    weight : f64
}

impl Environment {
    fn print_acc(&self) -> f64 {
        self.weight
    }

    fn weighted_sum_vector(&self, my_vec: &CxxVector<f64>)-> f64 {
        let my_sum : f64 =my_vec.iter().sum() ;
        my_sum*self.weight
    }
}