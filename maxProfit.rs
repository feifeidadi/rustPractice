// This is the Rust version of 'cppPractice/containers/maxProfit.C'

use std::cmp::{min, max};

fn max_profit(prices : Vec<i32>) -> i32
{
  if prices.is_empty()
  {
    return 0;
  }

  let mut _profit = 0;
  let mut _max_profit = 0;
  let mut _min_price = prices.first().copied().unwrap();
  for price in prices 
  {
    _min_price = min(_min_price, price);
    _profit = price - _min_price;
    _max_profit = max(_max_profit, _profit);
  }

  return _max_profit; 
}

fn main()
{
  println!("{}", max_profit(vec![]));
  println!("{}", max_profit(vec![1, 2, 3, 4, 5]));
  println!("{}", max_profit(vec![3, 10, 900, 1, 1000, 10]));
  println!("{}", max_profit(vec![100, 100, 100, 100, 100]));
}
