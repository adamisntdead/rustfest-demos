extern crate rgsl;

fn main() {
    let data : [f64; 5] = [17.2, 18.1, 16.5, 18.3, 12.6];

    let mean     = rgsl::statistics::mean(&data, 1, 5);
    let variance = rgsl::statistics::variance(&data, 1, 5);
    let largest  = rgsl::statistics::max(&data, 1, 5);
    let smallest = rgsl::statistics::min(&data, 1, 5);

    println!("The dataset is {}, {}, {}, {}, {}", data[0], data[1], data[2], data[3], data[4]);

    println!("The sample mean is {}", mean);
    println!("The estimated variance is {}", variance);
    println!("The largest value is {}", largest);
    println!("The smallest value is {}", smallest);
}