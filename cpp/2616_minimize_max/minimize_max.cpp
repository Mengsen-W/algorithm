#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimizeMax(vector<int>& nums, int p) {
    sort(nums.begin(), nums.end());
    auto check = [&](int mx) -> bool {
      int cnt = 0;
      for (int i = 0; i < nums.size() - 1; i++) {
        if (nums[i + 1] - nums[i] <= mx) {
          cnt++;
          i++;
        }
      }
      return cnt >= p;
    };
    int left = 0, right = nums.back() - nums[0];
    while (left < right) {
      int mid = (left + right) >> 1;
      if (check(mid)) {
        right = mid;
      } else {
        left = mid + 1;
      }
    }
    return left;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{10, 1, 2, 7, 1, 3}, 2, 1},
      {{4, 2, 1, 2}, 1, 0},
  };

  for (auto& [nums, p, ans] : tests) {
    assert(Solution().minimizeMax(nums, p) == ans);
  }
}