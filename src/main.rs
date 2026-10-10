use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone)]
struct MarketTradeOrder {
    trader_name: String,
    asset_quantity: u32,
    target_price: u32,
}

struct FinancialOrderBook {
    active_sell_asks_tree: BTreeMap<u32, VecDeque<MarketTradeOrder>>,
    
    active_buy_bids_tree: BTreeMap<u32, VecDeque<MarketTradeOrder>>,
}

impl FinancialOrderBook {
    fn initialize_empty_book() -> Self {
        FinancialOrderBook {
            active_sell_asks_tree: BTreeMap::new(),
            active_buy_bids_tree: BTreeMap::new(),
        }
    }

    fn submit_buy_order(&mut self, mut incoming_buy_order: MarketTradeOrder) {
        
        loop {
            let cheapest_available_sell_entry = self.active_sell_asks_tree.iter_mut().next();
            
            match cheapest_available_sell_entry {
                Some((&cheapest_market_price, active_sellers_queue)) => {
                    
                    if incoming_buy_order.target_price >= cheapest_market_price {
                        
                        let mut cheapest_seller_order = active_sellers_queue.pop_front().unwrap();
                        
                        let executed_trade_quantity = std::cmp::min(
                            incoming_buy_order.asset_quantity, 
                            cheapest_seller_order.asset_quantity
                        );
                        
                        println!("💰 TRADE EXECUTED: [{}] bought {} units from [{}] at ${}", 
                            incoming_buy_order.trader_name, 
                            executed_trade_quantity, 
                            cheapest_seller_order.trader_name, 
                            cheapest_market_price
                        );
                        
                        incoming_buy_order.asset_quantity -= executed_trade_quantity;
                        cheapest_seller_order.asset_quantity -= executed_trade_quantity;
                        
                        if cheapest_seller_order.asset_quantity > 0 {
                            active_sellers_queue.push_front(cheapest_seller_order);
                        }
                        
                        if active_sellers_queue.is_empty() {
                            self.active_sell_asks_tree.remove(&cheapest_market_price);
                        }
                        
                        if incoming_buy_order.asset_quantity == 0 {
                            break;
                        }
                    } else {
                        self.add_to_buy_waitlist(incoming_buy_order);
                        break;
                    }
                }
                None => {
                    self.add_to_buy_waitlist(incoming_buy_order);
                    break;
                }
            }
        }
    }

    fn add_to_buy_waitlist(&mut self, leftover_buy_order: MarketTradeOrder) {
        println!("📥 Added to Buy Book: [{}] wants {} units at ${}", 
            leftover_buy_order.trader_name, 
            leftover_buy_order.asset_quantity, 
            leftover_buy_order.target_price
        );
        
        let target_price_queue = self.active_buy_bids_tree
            .entry(leftover_buy_order.target_price)
            .or_insert_with(VecDeque::new);
            
        target_price_queue.push_back(leftover_buy_order);
    }
    
    fn add_to_sell_waitlist(&mut self, new_sell_order: MarketTradeOrder) {
        println!("📤 Added to Sell Book: [{}] is selling {} units at ${}", 
            new_sell_order.trader_name, 
            new_sell_order.asset_quantity, 
            new_sell_order.target_price
        );
        let target_price_queue = self.active_sell_asks_tree
            .entry(new_sell_order.target_price)
            .or_insert_with(VecDeque::new);
        target_price_queue.push_back(new_sell_order);
    }
}

fn main() {
    let mut nasdaq_matching_engine = FinancialOrderBook::initialize_empty_book();

    println!("--- MARKET OPENS ---");

    nasdaq_matching_engine.add_to_sell_waitlist(MarketTradeOrder { trader_name: String::from("Alice"), asset_quantity: 10, target_price: 105 });
    nasdaq_matching_engine.add_to_sell_waitlist(MarketTradeOrder { trader_name: String::from("Bob"),   asset_quantity: 50, target_price: 100 });
    nasdaq_matching_engine.add_to_sell_waitlist(MarketTradeOrder { trader_name: String::from("Charlie"),asset_quantity: 20, target_price: 100 });

    println!("\n--- BUYER ENTERS THE MARKET ---");
    
    nasdaq_matching_engine.submit_buy_order(MarketTradeOrder { 
        trader_name: String::from("Whale_Investor"), 
        asset_quantity: 65, 
        target_price: 110 
    });
}