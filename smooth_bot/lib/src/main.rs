pub struct bd {
    pub X: f64,
    pub Y: f64,
    pub K: i128,
    pub collateral: f64,
    pub reserve: f64,
}
trait limit_buy { fn limit_buy(avg_price: f64, quantity: f64); }
trait limit_sell { fn limit_sell(avg_price: f64, quantity: f64); }
impl limit_buy for bd {
    fn limit_buy(avg_price: f64, quantity: f64) { // use collateral to buyback 20% average price paid
        println!("Issuing a limit buy order of {} $USDT @ ${}", quantity, (avg_price*0.8));
    }
}

impl limit_sell for bd {
    fn limit_sell(avg_price: f64, quantity: f64) {
        println!("Issuing a limit sell order of {} $SMT @ ${}", quantity, (avg_price*1.2));
    }
}

impl bd {
    pub fn new(X: f64, Y: f64, K: i128, reserve: f64) -> Self {
        Self {
            X,
            Y,
            K,
            collateral: 0.0,
            reserve,
        }
    }
    pub fn price_of_Y(&mut self) -> f64 { // what is the price of Y in comparison to X
        let price = (self.X / self.Y) as f64;
        return price;
    }
    pub fn price_of_X(&mut self) -> f64 { // what is the price of X in comparison to Y
        let price = (self.Y / self.X) as f64;
        return price;
    }
    pub fn swap_X_Y(&mut self, x: f64) -> f64 { // how many X ($USDT) will be swapped to Y ($SMT), return quantity of Y that was swapped
        self.collateral += x/2.0;
        self.X += x/2.0;
        let y_initial = self.Y;
        self.Y = (self.K / self.X as i128) as f64;
        self.reserve -= (self.K / self.X as i128) as f64 / 2.0;
        println!("Investor is buying {} $SMT with ${} of USDT for an average price of ${} per $SMT\n", (y_initial - self.Y)*2.0, x, (x/((y_initial - self.Y)*2.0)));
        bd::limit_buy((x/((y_initial - self.Y)*2.0)), x / 2.0);
        println!("There is ${} $USDT of collateral left and {} of $SMT left in reserve", self.collateral, self.reserve);
        return self.Y
    }
    pub fn swap_Y_X(&mut self, y: f64) -> f64 { // how many Y ($SMT) will be swapped to X ($USDT), return quantity of X (wrong) that was swapped
        self.Y +=  y;
        let x_initial = self.X;
        self.X = (self.K / self.Y as i128) as f64;
        println!("Investor is selling {} $SMT for ${} of USDT for an average price of ${} per $SMT\n", y, (x_initial - self.X), (self.X/(self.Y - y)));
        bd::limit_sell(self.X/(self.Y - y), y / 2.0);
        println!("There is ${} $USDT of collateral left and {} of $SMT left in reserve", self.collateral, self.reserve);
        return self.Y
    }
    pub fn get_collateral_value(&mut self) -> f64 {
        return self.collateral
    }
}