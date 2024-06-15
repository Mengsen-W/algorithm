#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumBeauty(vector<int>& nums, int k) {
    int m = *max_element(nums.begin(), nums.end());
    vector<int> diff(m + 2);
    for (int x : nums) {
      diff[max(x - k, 0)]++;
      diff[min(x + k + 1, m + 1)]--;
    }
    int res = 0, count = 0;
    for (int x : diff) {
      count += x;
      res = max(res, count);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{4, 6, 1, 2}, 2, 3},
      {{1, 1, 1, 1}, 10, 4},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().maximumBeauty(nums, k) == ans);
  }
}