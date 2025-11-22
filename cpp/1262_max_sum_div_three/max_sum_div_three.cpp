#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxSumDivThree(vector<int>& nums) {
    vector<int> f = {0, INT_MIN, INT_MIN};
    for (int num : nums) {
      vector<int> g = f;
      for (int i = 0; i < 3; ++i) {
        g[(i + num % 3) % 3] = max(g[(i + num % 3) % 3], f[i] + num);
      }
      f = std::move(g);
    }
    return f[0];
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{3, 6, 5, 1, 8}, 18},
      {{4}, 0},
      {{1, 2, 3, 4, 4}, 12},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().maxSumDivThree(nums) == ans);
  }
}