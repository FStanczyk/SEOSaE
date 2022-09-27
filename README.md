# SEOSaE
SEOSaE -> Stock Exchange Orderbook Simulator and Engine (light)

Start application with ```cargo run```

By default, app will start as a Simulation with randomly generated orders.
If you want to import your own data use data.csv file and change ```ORDERS_FROM_FILE``` to true.

You are able to manipulate some constant variables like:
  * ```STANDARD_DEVIATION``` - the higher the less matchings
  * ```REFRESH_FREQUENCY``` - (it's not really frequency): specifies the time between refreshes in milliseconds
  * ```STARTING_LOWEST_ASK``` and ```STARTING_HIGHEST_BID``` (LA, HB) - best if the difference not bigger than 0.2
  * ```SHOW_FULL_ORDER_BOOK``` - Instead of showing all amount for one price (sum of all amount from one price),if ```true``` it will show all orders not 
  suming up amounts.
  * ```DEBUG_MODE``` - if true, will print more info in stdout  
  * ```ORDERS_FROM_FILE``` - If true, the generator will not work and the program will only process data given in data.csv file.
 
 
