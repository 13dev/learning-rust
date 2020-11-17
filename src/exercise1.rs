use rand::Rng;

struct Machine {
    coins: [f32; 8],
    bills: [u16; 7],
}

impl Machine  {
    fn new() -> Self {
        Self {
            coins: [0.01, 0.02, 0.05, 0.10, 0.20, 0.5, 1.0, 2.0],
            bills: [5, 10, 20, 50, 100, 200, 500]
        }
    }

    pub fn generate_value(high: u16) -> u16 {
        rand::thread_rng().gen_range(1, high)
    }


    fn make_exchange(&self, mut value: f32) -> Vec<f32> {
        let mut exchange: Vec<f32> = Vec::new();

        for &bill in self.bills.iter().rev() {
            let bill = bill as f32;

            if value >= bill {
                exchange.push(bill);
                value -= bill;
            }
        }

        for &coin in self.coins.iter().rev() {
            if value >= coin {
                exchange.push(coin);
                value -= coin;
            }
        }

        exchange
    }
}


pub fn testing() {
    let machine: Machine = Machine::new();
    let value: u16 = Machine::generate_value(50);

    println!("Random value: {}", value);
    let exchange = machine.make_exchange(value as f32);

    println!("Troco: {:?}", exchange);

}