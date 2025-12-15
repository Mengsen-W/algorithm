#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  long long getDescentPeriods(vector<int>& prices) {
    int n = prices.size();
    long long res = 1;
    int prev = 1;
    for (int i = 1; i < n; ++i) {
      if (prices[i] == prices[i - 1] - 1) {
        ++prev;
      } else {
        prev = 1;
      }
      res += prev;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{3, 2, 1, 4}, 7},
      {{8, 6, 7, 7}, 4},
      {{1}, 1},
  };

  for (auto& [prices, ans] : tests) {
    assert(Solution().getDescentPeriods(prices) == ans);
  }
}
