
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  double minimumAverage(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    int n = nums.size();
    double res = numeric_limits<double>::max();
    for (int i = 0; i < n / 2; i++) {
      res = min(res, (nums[i] + nums[n - 1 - i]) / 2.0);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, double>> tests{
      {{7, 8, 3, 4, 15, 13, 4, 1}, 5.5},
      {{1, 9, 8, 3, 10, 5}, 5.5},
      {{1, 2, 3, 7, 8, 9}, 5.0},
  };

  for (auto&[nums, ans] : tests) {
    assert(Solution().minimumAverage(nums) == ans);
  }
}