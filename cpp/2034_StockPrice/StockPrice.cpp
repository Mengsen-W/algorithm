/*
 * @Date: 2023-10-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-08
 * @FilePath: /algorithm/cpp/2034_StockPrice/StockPrice.cpp
 */

#include <cassert>
#include <set>
#include <unordered_map>

using namespace std;

class StockPrice {
 public:
  StockPrice() { this->maxTimestamp = 0; }

  void update(int timestamp, int price) {
    maxTimestamp = max(maxTimestamp, timestamp);
    int prevPrice = timePriceMap.count(timestamp) ? timePriceMap[timestamp] : 0;
    timePriceMap[timestamp] = price;
    if (prevPrice > 0) {
      auto it = prices.find(prevPrice);
      if (it != prices.end()) {
        prices.erase(it);
      }
    }
    prices.emplace(price);
  }

  int current() { return timePriceMap[maxTimestamp]; }

  int maximum() { return *prices.rbegin(); }

  int minimum() { return *prices.begin(); }

 private:
  int maxTimestamp;
  unordered_map<int, int> timePriceMap;
  multiset<int> prices;
};

int main() {
  StockPrice stockPrice = StockPrice();
  stockPrice.update(1, 10);            // 时间戳为 [1] ，对应的股票价格为 [10] 。
  stockPrice.update(2, 5);             // 时间戳为 [1,2] ，对应的股票价格为 [10,5] 。
  assert(stockPrice.current() == 5);   // 返回 5 ，最新时间戳为 2 ，对应价格为 5 。
  assert(stockPrice.maximum() == 10);  // 返回 10 ，最高价格的时间戳为 1 ，价格为 10 。
  stockPrice.update(1, 3);             // 之前时间戳为 1 的价格错误，价格更新为 3 。
                                       // 时间戳为 [1,2] ，对应股票价格为 [3,5] 。
  assert(stockPrice.maximum() == 5);   // 返回 5 ，更正后最高价格为 5 。
  stockPrice.update(4, 2);             // 时间戳为 [1,2,4] ，对应价格为 [3,5,2] 。
  assert(stockPrice.minimum() == 2);   // 返回 2 ，最低价格时间戳为 4 ，价格为 2 。
}