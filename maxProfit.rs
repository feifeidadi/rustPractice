// This is the Rust version of 'cppPractice/containers/maxProfit.C'

use std::cmp::{min, max};

fn max_profit(prices : &[i32]) -> i32
{
  if prices.is_empty()
  {
    return 0;
  }

  let mut max_profit = 0;
  let mut min_price = prices[0];
  for &price in prices 
  {
    min_price = min(min_price, price);
    max_profit = max(max_profit, price - min_price /* profit on current price */);
  }

  max_profit
}

fn main()
{
  println!("{}", max_profit(&[]));
  println!("{}", max_profit(&[1, 2, 3, 4, 5]));
  println!("{}", max_profit(&[3, 10, 900, 1, 1000, 10]));
  println!("{}", max_profit(&[100, 100, 100, 100, 100]));
}
