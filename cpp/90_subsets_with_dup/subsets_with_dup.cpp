#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> t;
  vector<vector<int>> ans;
  vector<vector<int>> subsetsWithDup(vector<int> &nums) {
    sort(nums.begin(), nums.end());
    int n = nums.size();
    for (int mask = 0; mask < (1 << n); ++mask) {
      t.clear();
      bool flag = true;
      for (int i = 0; i < n; ++i) {
        if (mask & (1 << i)) {
          if (i > 0 && (mask >> (i - 1) & 1) == 0 && nums[i] == nums[i - 1]) {
            flag = false;
            break;
          }
          t.push_back(nums[i]);
        }
      }
      if (flag) {
        ans.push_back(t);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<vector<int>>>> tests{
      {{1, 2, 2}, {{}, {1}, {1, 2}, {1, 2, 2}, {2}, {2, 2}}},
      {{0}, {{}, {0}}},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().subsetsWithDup(nums) == ans);
  }
}