use colored::Colorize;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{thread, time::Duration};
mod matching;
mod order;
mod read_file;

#[cfg(test)]
mod tests;
// The higher standard deviation the more diverse the orders will be, 0.1/0.2 keeps them in a reasonable range easy to track.
const STANDARD_DEVIATION: f64 = 0.1; // for the sake of generating prices via normal distribution

// Each refresh new order will be generated
const REFRESH_FREQUENCY: u64 = 1000; // in milliseconds

/*
 * Let's help the random order generator with some first starting values
 * Best difference would be not bigger than the standard deviation
*/
const STARTING_LOWEST_ASK: f64 = 117.6;
const STARTING_HIGHEST_BID: f64 = 117.4;

// Full order book takes every order as a single bid/ask and it's not summing up orders in order to
// see the supply of the asset
const SHOW_FULL_ORDER_BOOK: bool = false;

// if true, generator will not work and all the datawill be processed once and logout
const ORDERS_FROM_FILE: bool = false;
const PATH_TO_READ_FILE: &str = "./data.csv";

//Shows helpfull debugging data
const DEBUG_MODE: bool = false;

fn main() {
    //Initialize LA and HB
    let mut lowest_ask: f64 = STARTING_LOWEST_ASK; // price
    let mut highest_bid: f64 = STARTING_HIGHEST_BID;

    //Initialize orderbook vectors
    let mut buy_orders: Vec<order::Order> = vec![]; // (price, amount)
    let mut sell_orders: Vec<order::Order> = vec![];

    /*
     * channels to pass order info from one thread to another
     *
     * @params       bool - is it a buy_order: true/false
     *         (f64, u32) - tuple of (price, amount)
     */
    let (order_sender, order_receiver): (Sender<order::Order>, Receiver<order::Order>) = channel();

    // We use a channel to suspend the generator thread loop so it's not constantly working, only when needed.
    let (let_work_sender, let_work_receiver): (Sender<bool>, Receiver<bool>) = channel();

    // ? Do I need to use channels fo HB and LA?
    //let (hb_sender, hb_receiver): (Sender<f64>, Receiver<f64>) = channel(); // hb -> Highest bid
    //let (la_sender, la_receiver): (Sender<f64>, Receiver<f64>) = channel(); // la -> Lowest ask

    read_file::clear_logs().expect("unable to clear logs");

    /*
     * This thread is responsible for managing the order book and matching
     */
    let orderbook_thread = std::thread::spawn(move || {
        loop {
            clear();

            if !ORDERS_FROM_FILE {
                let _ = let_work_sender.send(true);
                let mut new_order = order_receiver.recv().unwrap(); //Gets an order from generator thread.
                let _ = let_work_sender.send(false);

                /*
                 * Checks if the order is a buy or sell order and finds a matching order in
                 * correct vector.
                 */
                if new_order.order_type == order::TYPE_OF_ORDER::BUY {
                    if !sell_orders.is_empty() {
                        matching::matching(&mut sell_orders, &mut new_order);
                    }

                    if new_order.amount != 0 {
                        buy_orders.push(new_order);
                    }
                } else {
                    if !buy_orders.is_empty() {
                        matching::matching(&mut buy_orders, &mut new_order);
                    }

                    if new_order.amount != 0 {
                        sell_orders.push(new_order);
                    }
                }
            } else {
                read_file::process_data(&mut buy_orders, &mut sell_orders)
                    .expect("Error while reading data from file");
            }

            /*
             * Here we sort vecotrs so the orders log in a reasoable way.
             *
             * We don't actually need to sort a vector each time, it would be much more efficient
             * to find a correct place to insert a new order. On a list todo
             */
            buy_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
            sell_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

            /*
             * here we upgrade the LA and HB, we need to check if vector is not empty
             * because there might be situation when all orders are fulfilled.
             *
             * There would be a better way if program would upgrade HB and LA only when
             * it changed, not every loop.
             */
            if !sell_orders.is_empty() {
                lowest_ask = sell_orders[0].price;
            }

            if !buy_orders.is_empty() {
                highest_bid = buy_orders[0].price;
            }

            if SHOW_FULL_ORDER_BOOK {
                println!("{}", "SELL_ORDERS: ".bright_red());
                order::display_orders(&sell_orders);
                println!("{}", "BUY_ORDERS: ".bright_green());
                order::display_orders(&buy_orders);
                println!("{}{:?}", "Lowest ask: ".on_blue(), lowest_ask);
                println!("{}{:?}", "Highest bid: ".on_blue(), highest_bid);
            } else {
                order::display_orderbook(&sell_orders, &buy_orders, &lowest_ask, &highest_bid);
            }

            if ORDERS_FROM_FILE {
                break;
            } else {
                slp();
            }
        }
    });

    /*
     * Generator thread
     * This thread will be responsible for generating new orders
     */
    if !ORDERS_FROM_FILE {
        std::thread::spawn(move || loop {
            if let_work_receiver.recv().unwrap() {
                DEBUG_MODE_DATA("Generator thread working...");

                let new_order = order::Order::generate_order(&lowest_ask, &highest_bid);

                let _ = order_sender.send(new_order);
            }
        });
    }

    orderbook_thread.join().unwrap();
}

/*
 * Function to clear the screen
 */
fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

/*
 * Function to sleep the thread for a REFRESH_FREQUENCY amount of time
*/
fn slp() {
    thread::sleep(Duration::from_millis(REFRESH_FREQUENCY));
}

#[allow(non_snake_case)]
fn DEBUG_MODE_DATA(text: &str) {
    if DEBUG_MODE {
        println!("{}", text);
    }
}
