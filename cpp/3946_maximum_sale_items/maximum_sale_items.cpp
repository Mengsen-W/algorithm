#include <cassert>
#include <climits>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int maximumSaleItems(vector<vector<int>>& items, int budget) {
    vector<int> f(budget + 1, 0);
    int mn = INT_MAX;
    for (const auto& item : items) {
      int factor = item[0], price = item[1];
      mn = min(mn, price);
      int cnt = 0;
      for (const auto& other : items) {
        if (other[0] % factor == 0) {
          cnt++;
        }
      }
      for (int j = budget; j >= price; j--) {
        f[j] = max(f[j], f[j - price] + cnt);
      }
    }
    int ans = 0;
    for (int j = 0; j <= budget; j++) {
      ans = max(ans, f[j] + (budget - j) / mn);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int>> tests{
      {{{6, 2}, {2, 6}, {3, 4}}, 9, 4},
      {{{2, 4}, {3, 2}, {4, 1}, {6, 4}, {12, 4}}, 8, 10},
  };

  for (auto [items, budget, expected] : tests) {
    assert(Solution().maximumSaleItems(items, budget) == expected);
  }
}
