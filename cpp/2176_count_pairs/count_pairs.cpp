#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countPairs(vector<int>& nums, int k) {
    int n = nums.size();
    int res = 0;  // 符合要求数对个数
    for (int i = 0; i < n - 1; ++i) {
      for (int j = i + 1; j < n; ++j) {
        if ((i * j) % k == 0 && nums[i] == nums[j]) {
          ++res;
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{3, 1, 2, 2, 2, 1, 3}, 2, 4},
      {{1, 2, 3, 4}, 1, 0},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().countPairs(nums, k) == ans);
  }
}