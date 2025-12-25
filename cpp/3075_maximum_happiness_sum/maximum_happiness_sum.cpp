#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maximumHappinessSum(vector<int>& happiness, int k) {
    sort(happiness.begin(), happiness.end(), greater<int>());
    long long ans = 0;
    for (int i = 0; i < k; ++i) {
      ans += max(happiness[i] - i, 0);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{1, 2, 3}, 2, 4},
      {{1, 1, 1, 1}, 2, 1},
      {{2, 3, 4, 5}, 1, 5},
  };

  for (auto& [happiness, k, expect] : tests) {
    assert(Solution().maximumHappinessSum(happiness, k) == expect);
  }
}