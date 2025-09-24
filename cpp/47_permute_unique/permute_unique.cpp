#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
  vector<int> vis;

 public:
  void backtrack(vector<int>& nums, vector<vector<int>>& ans, int idx, vector<int>& perm) {
    if (idx == nums.size()) {
      ans.emplace_back(perm);
      return;
    }
    for (int i = 0; i < (int)nums.size(); ++i) {
      if (vis[i] || (i > 0 && nums[i] == nums[i - 1] && !vis[i - 1])) {
        continue;
      }
      perm.emplace_back(nums[i]);
      vis[i] = 1;
      backtrack(nums, ans, idx + 1, perm);
      vis[i] = 0;
      perm.pop_back();
    }
  }

  vector<vector<int>> permuteUnique(vector<int>& nums) {
    vector<vector<int>> ans;
    vector<int> perm;
    vis.resize(nums.size());
    sort(nums.begin(), nums.end());
    backtrack(nums, ans, 0, perm);
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<vector<int>>>> tests{
      {{1, 1, 2}, {{1, 1, 2}, {1, 2, 1}, {2, 1, 1}}},
      {{1, 2, 3}, {{1, 2, 3}, {1, 3, 2}, {2, 1, 3}, {2, 3, 1}, {3, 1, 2}, {3, 2, 1}}},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().permuteUnique(nums) == ans);
  }
}