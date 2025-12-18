#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maxProfit(vector<int>& prices, vector<int>& strategy, int k) {
    int n = prices.size();
    vector<long long> profitSum(n + 1);
    vector<long long> priceSum(n + 1);
    for (int i = 0; i < n; i++) {
      profitSum[i + 1] = profitSum[i] + prices[i] * strategy[i];
      priceSum[i + 1] = priceSum[i] + prices[i];
    }
    long long res = profitSum[n];
    for (int i = k - 1; i < n; i++) {
      long long leftProfit = profitSum[i - k + 1];
      long long rightProfit = profitSum[n] - profitSum[i + 1];
      long long changeProfit = priceSum[i + 1] - priceSum[i - k / 2 + 1];
      res = max(res, leftProfit + changeProfit + rightProfit);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int, long long>> tests{
      {{4, 2, 8}, {-1, 0, 1}, 2, 10},
      {{5, 4, 3}, {1, 1, 0}, 2, 9},
  };

  for (auto& [prices, strategy, k, expect] : tests) {
    assert(Solution().maxProfit(prices, strategy, k) == expect);
  }
}
