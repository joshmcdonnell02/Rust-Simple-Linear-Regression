use std::iter::FromIterator;

use ml::LinearRegression;

// y = ax + b
// NYA,2014-10-27,10547.66016,10559.75977,10483.75977,10544.41992,10544.41992,3538860000
// 10544.41992 = a(10547.66016) + b
// a = coefficient
// b = Intercept


fn main() {

    let linear_model = &mut ml::LinearRegression{
        coefficient: 0.0,
        intercept: 0.0,
    };

    let data = ml::read_data().unwrap();

    let x_train = Vec::from_iter(data[0][0..1000].iter().cloned());
    let y_train = Vec::from_iter(data[1][0..1000].iter().cloned());

    let x_test = Vec::from_iter(data[0][1000..1300].iter().cloned());
    let y_test = Vec::from_iter(data[1][1000..1300].iter().cloned());

    linear_model.fit(&x_train, &y_train);

    let pred = linear_model.predict(&x_test);
    let score = linear_model.r2(&y_test, &pred);

    println!("FInal R^2 Score: {}", score);
    println!("Coefficient: {}, intercept: {}", linear_model.coefficient, linear_model.intercept)
}