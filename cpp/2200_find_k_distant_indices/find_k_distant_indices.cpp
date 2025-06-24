#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findKDistantIndices(vector<int>& nums, int key, int k) {
    vector<int> res;
    int r = 0;  // 未被判断过的最小下标
    int n = nums.size();
    for (int j = 0; j < n; ++j) {
      if (nums[j] == key) {
        int l = max(r, j - k);
        r = min(n - 1, j + k) + 1;
        for (int i = l; i < r; ++i) {
          res.push_back(i);
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, vector<int>>> tests{
      {{3, 4, 9, 1, 3, 9, 5}, 9, 1, {1, 2, 3, 4, 5, 6}},
      {{2, 2, 2, 2, 2}, 2, 2, {0, 1, 2, 3, 4}},
  };

  for (auto& [nums, key, k, ans] : tests) {
    assert(Solution().findKDistantIndices(nums, key, k) == ans);
  }
}