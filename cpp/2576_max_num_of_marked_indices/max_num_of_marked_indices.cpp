#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxNumOfMarkedIndices(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    int n = nums.size();
    int l = 0, r = n / 2;
    auto check = [&](int m) {
      for (int i = 0; i < m; i++) {
        if (nums[i] * 2 > nums[n - m + i]) {
          return false;
        }
      }
      return true;
    };
    while (l < r) {
      int m = l + r + 1 >> 1;
      if (check(m)) {
        l = m;
      } else {
        r = m - 1;
      }
    }
    return l * 2;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{3, 5, 2, 4}, 2},
      {{9, 2, 5, 4}, 4},
      {{7, 6, 8}, 0},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().maxNumOfMarkedIndices(nums) == ans);
  }
}