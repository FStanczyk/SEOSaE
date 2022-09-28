use std::{thread, time::Duration};
use colored::Colorize;
use std::{sync::{mpsc::{channel, Sender, Receiver}}};
use rand::{self, Rng};
use rand_distr::{Normal, Distribution};
mod read_file;
mod matching;

#[cfg(test)]
mod tests;
// The higher standard deviation the more diverse the orders will be, 0.1/0.2 keeps them in a reasonable range easy to track.
const STANDARD_DEVIATION: f64 = 0.2; // for the sake of generating prices via normal distribution

// Each refresh new order will be generated
const REFRESH_FREQUENCY: u64 = 1000; // in milliseconds

/*
 * Let's help the random order generator with some first starting values
 * Best difference would be not bigger than the standard deviation
*/ const STARTING_LOWEST_ASK: f64 = 117.6;
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
    let mut lowest_ask: f64  = STARTING_LOWEST_ASK; // price
    let mut highest_bid: f64 = STARTING_HIGHEST_BID;

    //Initialize orderbook vectors
    let mut buy_orders: Vec<(f64, u32)> = vec![]; // (price, amount) 
    let mut sell_orders: Vec<(f64, u32)> = vec![];


    /*
     * channels to pass order info from one thread to another
     * 
     * @params       bool - is it a buy_order: true/false
     *         (f64, u32) - tuple of (price, amount) 
    */
    let (order_sender, order_receiver): (Sender<(bool, (f64, u32))>, Receiver<(bool, (f64,u32))>) = channel();
    
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
                if new_order.0 {
                    if !sell_orders.is_empty() {
                        matching::matching(&mut sell_orders, &mut new_order.1);
                    }

                    if new_order.1.1 != 0 {
                        buy_orders.push(new_order.1);
                    }
                }else{
                    if !buy_orders.is_empty() {
                        matching::matching(&mut buy_orders, &mut new_order.1);
                    }

                    if new_order.1.1 != 0 {
                    sell_orders.push(new_order.1);
                    }
                }
            }else{

                read_file::process_data(&mut buy_orders, &mut sell_orders).expect("Error while reading data from file");
            }
            
            /*
             * Here we sort vecotrs so the orders log in a reasoable way.
             *  
             * We don't actually need to sort a vector each time, it would be much more efficient 
             * to find a correct place to insert a new order. On a list todo
            */
            buy_orders.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            sell_orders.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());


            /*
             * here we upgrade the LA and HB, we need to check if vector is not empty 
             * because there might be situation when all orders are fulfilled.
             * 
             * There would be a better way if program would upgrade HB and LA only when
             * it changed, not every loop.
            */
            if !sell_orders.is_empty() {
                lowest_ask = sell_orders[0].0;

            }

            if !buy_orders.is_empty() {
                highest_bid = buy_orders[0].0;
            }

            
            if SHOW_FULL_ORDER_BOOK{
                println!("{}{:?}","SELL_ORDERS: ".bright_red(), sell_orders);
                println!("{}{:?}","BUY_ORDERS: ".bright_green(), buy_orders);
                println!("{}{:?}","Lowest ask: ".on_blue(), lowest_ask);
                println!("{}{:?}","Highest bid: ".on_blue(), highest_bid);
            }else{
                display_orderbook(&sell_orders, &buy_orders, &lowest_ask, &highest_bid);
            }
            
            
            if ORDERS_FROM_FILE{break;}else{
                slp();
            }
        }
    });


    /*
     * Generator thread
     * This thread will be responsible for generating new orders
    */
    std::thread::spawn(move||{
        loop {       
            
                if !ORDERS_FROM_FILE && let_work_receiver.recv().unwrap(){

                    DEBUG_MODE_DATA("Generator thread working...");
                    
                    let new_order = generate_order(&lowest_ask, &highest_bid);
                    
                    let _ = order_sender.send(new_order);
                }
            }
    });

    orderbook_thread.join().unwrap();
}


fn generate_order(_low: &f64, _high: &f64) -> (bool, (f64, u32)){
    DEBUG_MODE_DATA("Generating new order...");
    let mut rng = rand::thread_rng();

    /*
     * First, we randomly choose if it's a buy order or sell order
    */
    let buy_order =rand::random();
    
    let log_order_type: String;
    
    /* 
     * In order to have a realistic simulation, I used a normal distribution to
     * genereate order prices.
     * 
     * The mean (required to normal distribution) for buyers/sellers in this 
     * simulation will be a standard_deviation +- highest/lowest bid/ask 
    */ 
    let mean: f64;
    if buy_order{
        mean = _low - 2.00*(STANDARD_DEVIATION);
        log_order_type = String::from("BUY");
    } else {
        mean = _high + 2.00*(STANDARD_DEVIATION);
        log_order_type = String::from("SELL");
    }

    let normal_dist = Normal::new(mean, STANDARD_DEVIATION).unwrap();
    let price = (normal_dist.sample(&mut rand::thread_rng())*10.00).round()/10.00;
    
    
    /*
     * for generating amount of asset in order I used standard random generator.
     */
    let amount = rng.gen_range(10..1000);

    if DEBUG_MODE {println!("Generated buy_order: {}, price: {}, amount: {}", buy_order, price, amount)};
    DEBUG_MODE_DATA("Generating new order finished.");

    read_file::write_log(
        vec![
            String::from("generation"), 
            log_order_type, 
            format!("{}",price),
            format!("{}",amount)
            ]).expect("Couldn't write to log");

    (buy_order, (price, amount))
}

/*
 * Function to clear the screen
 */
fn clear(){
    std::process::Command::new("clear").status().unwrap();
}

/*
 * Function to sleep the thread for a REFRESH_FREQUENCY amount of time
*/
fn slp(){
    thread::sleep(Duration::from_millis(REFRESH_FREQUENCY));
}

fn display_orderbook(sell_orders: &Vec<(f64, u32)>, buy_orders: &Vec<(f64, u32)>, lowest_ask: &f64, highest_bid: &f64){
    let sell_orders_by_price: Vec<(f64, u32)> = tuple_sum_by_key(sell_orders);
    let buy_orders_by_price:  Vec<(f64, u32)> = tuple_sum_by_key(buy_orders);

    println!("{}{:?}","SELL ORDERS: ".bright_red(), sell_orders_by_price);
    println!();
    println!("{}{:?}","BUY ORDERS: ".bright_green(), buy_orders_by_price);
    println!("{}: {}","Lowest ask ".on_blue(), lowest_ask);
    println!("{}: {}","Highest bid".on_blue(), highest_bid);

}   


/*
 * This function is for displaying purposes. 
 * If in orderbook there are two orders with the same price, the amounts could be
 * added so it's easier to read supply.
 * (10.2, 800),(10.2, 900) = (10.2, 1700) 
*/
fn tuple_sum_by_key(book: &Vec<(f64, u32)>) -> Vec<(f64, u32)> {

    /*
     * Algorithm to sum up values with the same key in tuple
    */
    let mut result = vec![];
    if !book.is_empty() {
        let mut item = book[0];
        for &t in book.iter().skip(1) {
            if t.0 == item.0 {
                item.1 += t.1;
            } else {
                result.push(item);
                item = t;
            }
        }
        result.push(item);
    }
    result
}




#[allow(non_snake_case)]
fn DEBUG_MODE_DATA(text: &str){
    if DEBUG_MODE {
        println!("{}", text);
    }
}