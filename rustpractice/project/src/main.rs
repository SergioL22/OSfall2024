

const FARENHEIT: f64 = 32.0;
 

fn main() {
    let mut x = fahrenheit_to_celsisus(104.0);
    println!("{}", x);
    let array = [32.0,33.0,34.0,35.0,36.0,37.0];

    for y in array.iter() {
        println!("{}", fahrenheit_to_celsisus(*y));
    }
}

fn fahrenheit_to_celsisus(f: f64) -> f64{ 
    let result = (f - FARENHEIT)*5.0/9.0;
    return result;

}

fn celsius_to_fahrenheit(c:f64) -> f64 {
    let result = (c * 9.0/5.0) + FARENHEIT;
    return result;

}




