use crate::order;
use crate::read_file;
use colored::Colorize;
pub fn matching(orders: &mut Vec<order::Order>, new_order: &mut order::Order) {
    crate::DEBUG_MODE_DATA("Started matching...");

    let mut fulfilled_order_index: Vec<usize> = vec![]; // keeps track of the indexes of all the fulfilled orders
    let mut found_match = true; // flag that tells if iteration through the ordrerbook gave any matching (helps to tell when to stop looping)

    /*
     * The loop will search for matching as long as the order wasn't fulfilled
     * or it made a full iteration without any matching
     */

    while new_order.amount != 0 && found_match {
        for (index, i) in orders.iter_mut().enumerate() {
            if new_order.price == i.price && i.amount != 0 {
                println!(
                    "{}{}{} and {}{}",
                    "match! sold/bought: ".black().on_white(),
                    i.price,
                    i.amount,
                    new_order.price,
                    new_order.amount
                );
                read_file::write_log(vec![
                    String::from("Matching"),
                    format!("{}", new_order.price),
                    format!("{}", new_order.amount),
                    String::from("matched with"),
                    format!("{}", i.price),
                    format!("{}", i.amount),
                ])
                .unwrap();

                //check which order will fulfill and list it to be removed
                if i.amount > new_order.amount {
                    i.amount -= new_order.amount;
                    new_order.amount = 0;

                    crate::DEBUG_MODE_DATA("matching: New order fulfilled");
                } else if i.amount < new_order.amount {
                    new_order.amount -= i.amount;
                    i.amount = 0;
                    fulfilled_order_index.push(index); //@notice rust doesn't allow borrowing for the second time so we cannot remove
                                                       // an order right here, that would be much easier.
                    crate::DEBUG_MODE_DATA("matching: New order not fulfilled");
                } else if i.amount == new_order.amount {
                    new_order.amount = 0;
                    i.amount = 0;
                    fulfilled_order_index.push(index);
                }
            } else {
                found_match = false;
            }
        }
    }

    /*
     * at the end we reverse iterate through the orderbook and remove all the
     * listed fulfilled orders. We have to reverse the iteration so the indexes
     * won't mess up.
     */
    for i in fulfilled_order_index.iter().rev() {
        orders.remove(*i);
        crate::DEBUG_MODE_DATA("matching: removing item");
    }

    crate::DEBUG_MODE_DATA("Matching Finished...");
}
