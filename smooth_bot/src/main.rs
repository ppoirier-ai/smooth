use plotly::common::Title;
use plotly::layout::{Axis, Layout, Legend};
use plotly::{Plot, Scatter};
pub mod random_swap;
use boundingcurve::test;

fn main() {

    println!("testing random number is {}", random_swap::swap());
    boundingcurve::test();

    let layout = Layout::new()
        .title(Title::new("Price Chart Protected with MMaaS"))
        .x_axis(Axis::new().title(Title::new("Time")))
        .y_axis(Axis::new().title(Title::new("Price")))
        .legend(Legend::new());

    let x_values= vec![1, 2, 3, 4, 5];
    let y_values = vec![2, 4, 6, 8, 10];

    let scatter = Scatter::new(x_values, y_values).name("Token X - Price Chart");

    let mut plot = Plot::new();
    plot.add_trace(scatter);
    plot.set_layout(layout);
    plot.show();
}
