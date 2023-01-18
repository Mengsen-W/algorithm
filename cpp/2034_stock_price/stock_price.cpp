/*
 * @Date: 2022-01-23 14:44:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-23 14:54:56
 */

#include <cassert>
#include <queue>
#include <unordered_map>

using namespace std;

typedef pair<int, int> pii;

class StockPrice {
 public:
  StockPrice() { this->maxTimestamp = 0; }

  void update(int timestamp, int price) {
    maxTimestamp = max(maxTimestamp, timestamp);
    timePriceMap[timestamp] = price;
    pqMax.emplace(price, timestamp);
    pqMin.emplace(price, timestamp);
  }

  int current() { return timePriceMap[maxTimestamp]; }

  int maximum() {
    while (true) {
      int price = pqMax.top().first, timestamp = pqMax.top().second;
      if (timePriceMap[timestamp] == price) {
        return price;
      }
      pqMax.pop();
    }
  }

  int minimum() {
    while (true) {
      int price = pqMin.top().first, timestamp = pqMin.top().second;
      if (timePriceMap[timestamp] == price) {
        return price;
      }
      pqMin.pop();
    }
  }

 private:
  int maxTimestamp;
  unordered_map<int, int> timePriceMap;
  priority_queue<pii, vector<pii>, less<pii>> pqMax;
  priority_queue<pii, vector<pii>, greater<pii>> pqMin;
};

int main() {
  StockPrice stockPrice;
  stockPrice.update(1, 10);
  stockPrice.update(2, 5);
  assert(stockPrice.current() == 5);
  assert(stockPrice.maximum() == 10);
  stockPrice.update(1, 3);
  assert(stockPrice.maximum() == 5);
  stockPrice.update(4, 2);
  assert(stockPrice.minimum() == 2);
}