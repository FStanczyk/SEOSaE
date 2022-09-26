#[cfg(test)]
mod tests {
    use crate::matching;

    #[test]
    fn it_finds_correct_matches_order_is_bigger_than_supply() {

        let amount_to_buy = 600;
        let start_amount = 100;

        let mut sell_orders = vec![(18.0, 100), (18.7, start_amount), (17.8, 150)];
        let mut buy_orders = vec![(18.1, 80), (18.4, 10), (17.2, 250)];

        let mut new_buy_order = (18.7, amount_to_buy);

        matching::matching(&mut sell_orders, &mut new_buy_order);
        if new_buy_order.1 != 0 {
            buy_orders.push(new_buy_order);
        }

        assert_eq!(sell_orders,vec![(18.0, 100), (17.8, 150)]);
        assert_eq!(buy_orders,vec![(18.1, 80), (18.4, 10), (17.2, 250), (18.7, amount_to_buy - start_amount)]); 
    }

    #[test]
    fn it_finds_correct_matches_order_is_smaller_than_supply() {

        let amount_to_buy = 773;
        let start_amount = 1090;

        let mut sell_orders = vec![(18.0, 100), (18.7, start_amount), (17.8, 150)];
        let mut buy_orders = vec![(18.1, 80), (18.4, 10), (17.2, 250)];

        let mut new_buy_order = (18.7, amount_to_buy);

        matching::matching(&mut sell_orders, &mut new_buy_order);
        if new_buy_order.1 != 0 {
            buy_orders.push(new_buy_order);
        }

        assert_eq!(sell_orders,vec![(18.0, 100), (18.7, start_amount - amount_to_buy), (17.8, 150)]);
        assert_eq!(buy_orders,vec![(18.1, 80), (18.4, 10), (17.2, 250)]); 
    }

    #[test]
    fn it_finds_correct_matches_order_is_equal_to_supply() {

        let amount_to_buy = 70;
        let start_amount = 70;

        let mut sell_orders = vec![(18.0, 100), (18.7, start_amount), (17.8, 150)];
        let mut buy_orders = vec![(18.1, 80), (18.4, 10), (17.2, 250)];

        let mut new_buy_order = (18.7, amount_to_buy);

        matching::matching(&mut sell_orders, &mut new_buy_order);
        if new_buy_order.1 != 0 {
            buy_orders.push(new_buy_order);
        }

        assert_eq!(sell_orders,vec![(18.0, 100), (17.8, 150)]);
        assert_eq!(buy_orders,vec![(18.1, 80), (18.4, 10), (17.2, 250)]); 
    }


    #[test]
    fn matching_works_for_the_only_order_in_orderbook_order_smaller_than_supply() {
        let start_amount = 1170;
        let amount_to_buy = 860;
        let mut sell_orders = vec![(18.0, start_amount)];
        let mut buy_orders = vec![];

        let mut new_buy_order = (18.0, amount_to_buy); // more than supply 700>100
            
        matching::matching(&mut sell_orders, &mut new_buy_order); //new order is a buy order
        if new_buy_order.1 != 0 {
            buy_orders.push(new_buy_order);
        }

        assert_eq!(buy_orders, vec![]);  
        assert_eq!(sell_orders, vec![(18.0,start_amount - amount_to_buy)]); 


    }

    #[test]
    fn matching_works_for_the_only_order_in_orderbook_order_is_equal_to_supply() {
        let start_amount = 456;
        let amount_to_buy = 456;
        let mut sell_orders = vec![(18.0, start_amount)];
        let mut buy_orders = vec![];

        let mut new_buy_order = (18.0, amount_to_buy); // more than supply 700>100
            
        matching::matching(&mut sell_orders, &mut new_buy_order); //new order is a buy order
        if new_buy_order.1 != 0 {
            buy_orders.push(new_buy_order);
        }

        assert_eq!(buy_orders, vec![]);  
        assert_eq!(sell_orders, vec![]); 
    }

   // somehow this one doesn't work... looks like it's can't go out of the loop in matching
    #[test]
    fn matching_works_for_the_only_order_in_orderbook_order_bigger_than_supply() {
        let start_amount = 70;
        let amount_to_buy = 80;
        let mut sell_orders = vec![(18.0, start_amount)];
        let mut buy_orders = vec![];

        let mut new_buy_order = (18.0, amount_to_buy); // more than supply 700>100
            
        matching::matching(&mut sell_orders, &mut new_buy_order); //new order is a buy order
        if new_buy_order.1 != 0 {
            buy_orders.push(new_buy_order);
        }

        assert_eq!(sell_orders, vec![]); 
        assert_eq!(buy_orders, vec![(18.0, amount_to_buy - start_amount)]);  
        

    }

    #[test]
    fn it_finds_correct_matches_multiple_order_smaller_than_last() {

        let amount_to_buy = 70;
        let start_amount = 20;

        let mut sell_orders = vec![(18.0, 100),(18.7, start_amount),(18.7, start_amount),(18.7, start_amount),(18.7, start_amount),(17.8, 150)];
        let mut buy_orders = vec![(18.1, 80), (18.4, 10), (17.2, 250)];

        let mut new_buy_order = (18.7, amount_to_buy);

        matching::matching(&mut sell_orders, &mut new_buy_order);
        if new_buy_order.1 != 0 {
            buy_orders.push(new_buy_order);
        }

        assert_eq!(sell_orders,vec![(18.0, 100), (18.7, 4* start_amount - amount_to_buy), (17.8, 150)]);
        assert_eq!(buy_orders,vec![(18.1, 80), (18.4, 10), (17.2, 250)]); 
    }


    #[test]
    fn it_finds_correct_matches_multiple_order_bigger_than_last() {

        let amount_to_buy = 70;
        let start_amount = 20;

        let mut sell_orders = vec![(18.0, 100),(18.7, start_amount),(18.7, start_amount),(18.7, start_amount),(17.8, 150)];
        let mut buy_orders = vec![(18.1, 80), (18.4, 10), (17.2, 250)];

        let mut new_buy_order = (18.7, amount_to_buy);

        matching::matching(&mut sell_orders, &mut new_buy_order);
        if new_buy_order.1 != 0 {
            buy_orders.push(new_buy_order);
        }

        assert_eq!(sell_orders,vec![(18.0, 100), (17.8, 150)]);
        assert_eq!(buy_orders,vec![(18.1, 80), (18.4, 10), (17.2, 250), (18.7, amount_to_buy-3*start_amount)]); 
    }

    #[test]
    fn it_finds_correct_matches_multiple_order_equal_to_last() {

        let amount_to_buy = 60;
        let start_amount = 20;

        let mut sell_orders = vec![(18.0, 100),(18.7, start_amount),(18.7, start_amount),(18.7, start_amount),(17.8, 150)];
        let mut buy_orders = vec![(18.1, 80), (18.4, 10), (17.2, 250)];

        let mut new_buy_order = (18.7, amount_to_buy);

        matching::matching(&mut sell_orders, &mut new_buy_order);
        if new_buy_order.1 != 0 {
            buy_orders.push(new_buy_order);
        }

        assert_eq!(sell_orders,vec![(18.0, 100), (17.8, 150)]);
        assert_eq!(buy_orders,vec![(18.1, 80), (18.4, 10), (17.2, 250)]); 
    }
}