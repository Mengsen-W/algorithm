#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int maximumSaleItems(vector<vector<int>>& items, int budget) {
    sort(items.begin(), items.end(), [](const auto& a, const auto& b) { return a[1] < b[1]; });
    int mx = 0;
    for (const auto& item : items) {
      mx = max(mx, item[0]);
    }
    vector<int> freq(mx + 1, 0);
    for (const auto& item : items) {
      freq[item[0]]++;
    }
    for (int i = 1; i <= mx; i++) {
      if (freq[i] > 0) {
        for (int j = i * 2; j <= mx; j += i) {
          freq[i] += freq[j];
        }
      }
    }

    int mn = items[0][1];
    int ans = 0;
    for (const auto& item : items) {
      int factor = item[0], price = item[1];
      if (price - mn > mn) {
        break;
      }
      int cnt = freq[factor] - 1;
      int take = min(cnt, budget / price);
      ans += take * 2;
      budget -= take * price;
    }
    ans += budget / mn;
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int>> tests{
      {{{1, 6}, {2, 4}, {3, 5}}, 19, 5},
      {{{2, 8}, {1, 10}, {6, 6}, {4, 12}, {5, 20}, {5, 17}}, 35, 7},
  };

  for (auto [items, budget, expected] : tests) {
    assert(Solution().maximumSaleItems(items, budget) == expected);
  }
}