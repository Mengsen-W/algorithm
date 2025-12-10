#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countPermutations(vector<int>& complexity) {
    int n = complexity.size();
    if (*min_element(complexity.begin() + 1, complexity.end()) <= complexity[0]) {
      return 0;
    }

    int mod = 1000000007;
    int ans = 1;
    for (int i = 2; i < n; ++i) {
      ans = static_cast<long long>(ans) * i % mod;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3}, 2},
      {{3, 3, 3, 4, 4, 4}, 0},
  };

  for (auto& [complexity, expect] : tests) {
    assert(Solution().countPermutations(complexity) == expect);
  }
}
