//use plotly::common::Title;
//use plotly::layout::{Axis, Layout, Legend};
//use plotly::{Plot, Scatter};

pub mod random_swap;
use boundingcurve::{*};
use std::{thread, time};

fn main() {

    println!("testing random number is {}", random_swap::swap());
    let mut boundingcurve = boundingcurve::bd::new(1000.0, 500.0, (1000.0 * 500.0) as i128, 100000.0); // initialize a new bounding curve with X = 1000.0, Y = 500.0
    println!("INITIAL BOUNDING CURVE CREATED");
    println!("$USDT = {}, $SMT = {}, and K = {}, collateral = {}, and reserve = {}", boundingcurve.X, boundingcurve.Y, boundingcurve.K, boundingcurve.collateral, boundingcurve.reserve);
    println!("Price of $SMT is ${}\n", boundingcurve.price_of_Y());

    println!("---------------------------------------------------------------");
    println!("Investor A will be swapping $500 worth of $USDT to buy $SMT");
    boundingcurve.swap_X_Y(500.0); // investor A BUY $500.00 worth of $SMT

    println!("$USDT = {}, $SMT = {}, and K = {}, collateral = {}, and reserve = {}", boundingcurve.X, boundingcurve.Y, boundingcurve.K, boundingcurve.collateral, boundingcurve.reserve);
    println!("Price of $SMT is ${}\n", boundingcurve.price_of_Y());

    println!("---------------------------------------------------------------");
    println!("Investor A will be swapping 100 $SMT back to $USDT");
    boundingcurve.swap_Y_X(100.0); // investor A sell 100.00 of $SMT
    println!("$USDT = {}, $SMT = {}, and K = {}, collateral = {}, and reserve = {}", boundingcurve.X, boundingcurve.Y, boundingcurve.K, boundingcurve.collateral, boundingcurve.reserve);
    println!("Price of $SMT is ${}\n", boundingcurve.price_of_Y());

    //let layout = Layout::new()
    //    .title(Title::new("Price Chart Protected with MMaaS"))
    //    .x_axis(Axis::new().title(Title::new("Time")))
    //    .y_axis(Axis::new().title(Title::new("Price")))
    //    .legend(Legend::new());

    //let x_values= vec![1, 2, 3, 4, 5];
    //let y_values = vec![2, 4, 6, 8, 10];

    //let scatter = Scatter::new(x_values, y_values).name("Token X - Price Chart");

    //let mut plot = Plot::new();
    //plot.add_trace(scatter);
    //plot.set_layout(layout);
    //plot.show();

    thread::sleep(time::Duration::from_millis(100));



    //let x_values2= vec![5, 4, 3, 4, 5];
    //let y_values2 = vec![3, 4, 6, 8, 10];
    //let scatter2 = Scatter::new(x_values2, y_values2).name("Token X - Price Chart");

    //plot.add_trace(scatter2);
    //plot.show();

}
