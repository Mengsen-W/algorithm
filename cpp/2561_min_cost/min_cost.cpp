#include <algorithm>
#include <cassert>
#include <climits>
#include <numeric>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    long long minCost(vector<int>& basket1, vector<int>& basket2) {
        int m = INT_MAX;
        unordered_map<int, int> frequency_map;
        for (int b1 : basket1) {
            frequency_map[b1]++;
            m = min(m, b1);
        }
        for (int b2 : basket2) {
            frequency_map[b2]--;
            m = min(m, b2);
        }
        vector<int> merge;
        for (auto [k, c] : frequency_map) {
            if (c % 2 != 0) {
                return -1;
            }
            for (int i = 0; i < abs(c) / 2; ++i) {
                merge.push_back(k);
            }
        }
        nth_element(merge.begin(), merge.begin() + merge.size() / 2, merge.end());
        return accumulate(merge.begin(), merge.begin() + merge.size() / 2, 0ll,
            [&](long long res, int x) -> long long {
                return res + min(2 * m, x);
            }
        );
    }
};

int main() {
  vector<tuple<vector<int>, vector<int>, long long>> tests{
      {{4, 2, 2, 2}, {1, 4, 1, 2}, 1},
      {{2, 3, 4, 1}, {3, 2, 5, 1}, -1},
  };

  for (auto [basket1, basket2, expected] : tests) {
    assert(Solution().minCost(basket1, basket2) == expected);
  }
}