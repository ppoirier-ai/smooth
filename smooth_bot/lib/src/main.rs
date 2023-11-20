pub struct bd {
    pub X: f64,
    pub Y: f64,
    pub K: i128,
}

impl bd {
    pub fn new(X: f64, Y: f64, K: i128) -> Self {
        Self {
            X,
            Y,
            K,
        }
    }
    pub fn price_of_Y(self) -> f64 { // what is the price of Y in comparison to X
        let price = (self.X / self.Y) as f64;
        return price;
    }
    pub fn price_of_X(self) -> f64 { // what is the price of X in comparison to Y
        let price = (self.Y / self.X) as f64;
        return price;
    }
    pub fn swap_X_Y(&mut self, x: f64) -> f64 { // how many X will be swapped to Y, return quantity of Y that was swapped
        self.X += x;
        self.Y = (self.K / self.X as i128) as f64;
        return self.Y
    }
    pub fn swap_Y_X(&mut self, y: f64) -> f64 { // how many Y will be swapped to X, return quantity of X that was swapped
        self.Y +=  y;
        self.X = (self.K / self.Y as i128) as f64;
        return self.X
    }
}