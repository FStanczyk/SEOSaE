use crate::read_file;
use crate::{DEBUG_MODE, DEBUG_MODE_DATA, STANDARD_DEVIATION};
use colored::Colorize;
use rand::{self, Rng};
use rand_distr::{Distribution, Normal};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TYPE_OF_ORDER {
    SELL,
    BUY,
}
#[derive(Clone, Copy, Debug)]
pub struct Order {
    pub order_type: TYPE_OF_ORDER,
    pub price: f64,
    pub amount: u32,
}

impl Order {
    pub fn generate_order(_low: &f64, _high: &f64) -> Self {
        DEBUG_MODE_DATA("Generating new order...");
        let mut rng = rand::thread_rng();

        /*
         * First, we randomly choose if it's a buy order or sell order
         * In order to have a realistic simulation, I used a normal distribution to
         * genereate order prices.
         *
         * The mean (required to normal distribution) for buyers/sellers in this
         * simulation will be a standard_deviation +- highest/lowest bid/ask
         */
        let order_type: TYPE_OF_ORDER;
        let log_order_type: String;
        let mean: f64;
        match rand::random() {
            false => {
                order_type = TYPE_OF_ORDER::SELL;
                mean = _high + 2.00 * (STANDARD_DEVIATION);
                log_order_type = String::from("SELL");
            }
            true => {
                order_type = TYPE_OF_ORDER::BUY;
                mean = _low - 2.00 * (STANDARD_DEVIATION);
                log_order_type = String::from("BUY");
            }
        }

        let normal_dist = Normal::new(mean, STANDARD_DEVIATION).unwrap();
        let price = (normal_dist.sample(&mut rand::thread_rng()) * 10.00).round() / 10.00;

        /*
         * for generating amount of asset in order I used standard random generator.
         */
        let amount = rng.gen_range(10..1000);

        if DEBUG_MODE {
            println!(
                "Generated buy_order: {}, price: {}, amount: {}",
                log_order_type, price, amount
            )
        };
        DEBUG_MODE_DATA("Generating new order finished.");

        read_file::write_log(vec![
            String::from("generation"),
            log_order_type,
            format!("{}", price),
            format!("{}", amount),
        ])
        .expect("Couldn't write to log");

        // (buy_order, (price, amount))
        Self {
            order_type,
            price,
            amount,
        }
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.order_type == other.order_type
            && self.price == other.price
            && self.amount == other.amount
    }
}

pub fn display_orders(orderbook: &[Order]) {
    for i in orderbook.iter() {
        print!("[{}, {}], ", i.price, i.amount)
    }
    println!();
}

pub fn display_orderbook(
    sell_orders: &[Order],
    buy_orders: &[Order],
    lowest_ask: &f64,
    highest_bid: &f64,
) {
    let sell_orders_by_price: Vec<Order> = tuple_sum_by_key(sell_orders);
    let buy_orders_by_price: Vec<Order> = tuple_sum_by_key(buy_orders);

    print!("{}", "SELL ORDERS: ".bright_red());
    display_orders(&sell_orders_by_price);
    println!();
    print!("{}", "BUY ORDERS: ".bright_green());
    display_orders(&buy_orders_by_price);
    println!("{}: {}", "Lowest ask ".on_blue(), lowest_ask);
    println!("{}: {}", "Highest bid".on_blue(), highest_bid);
}

/*
 * This function is for displaying purposes.
 * If in orderbook there are two orders with the same price, the amounts could be
 * added so it's easier to read supply.
 * (10.2, 800),(10.2, 900) = (10.2, 1700)
*/
fn tuple_sum_by_key(book: &[Order]) -> Vec<Order> {
    /*
     * Algorithm to sum up values with the same key in tuple
     */
    let mut result = vec![];
    if !book.is_empty() {
        let mut item = book[0];
        for &t in book.iter().skip(1) {
            if t.price == item.price {
                item.amount += t.amount;
            } else {
                result.push(item);
                item = t;
            }
        }
        result.push(item);
    }
    result
}
