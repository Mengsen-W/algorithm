#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> maxSubsequence(vector<int>& nums, int k) {
    int n = nums.size();
    vector<pair<int, int>> vals;  // 辅助数组
    for (int i = 0; i < n; ++i) {
      vals.emplace_back(i, nums[i]);
    }
    // 按照数值降序排序
    sort(vals.begin(), vals.end(), [&](auto x1, auto x2) { return x1.second > x2.second; });
    // 取前 k 个并按照下标升序排序
    sort(vals.begin(), vals.begin() + k);
    vector<int> res;  // 目标子序列
    for (int i = 0; i < k; ++i) {
      res.push_back(vals[i].second);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<int>>> tests{
      {{2, 1, 3, 3}, 2, {3, 3}},
      {{-1, -2, 3, 4}, 3, {-1, 3, 4}},
      {{3, 4, 3, 3}, 2, {3, 4}},
  };

  for (auto& [nums, k, expected] : tests) {
    assert(expected == Solution().maxSubsequence(nums, k));
  }
  return 0;
}