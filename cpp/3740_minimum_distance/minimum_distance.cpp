#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int minimumDistance(vector<int>& nums) {
    int n = nums.size();
    int ans = n + 1;

    for (int i = 0; i < n - 2; i++) {
      for (int j = i + 1; j < n - 1; j++) {
        if (nums[i] != nums[j]) {
          continue;
        }
        for (int k = j + 1; k < n; k++) {
          if (nums[j] == nums[k]) {
            ans = std::min(ans, k - i);
            break;
          }
        }
      }
    }

    return ans == n + 1 ? -1 : ans * 2;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 1, 1, 3}, 6},
      {{1, 1, 2, 3, 2, 1, 2}, 8},
      {{1}, -1},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().minimumDistance(nums) == ans);
  }
}
