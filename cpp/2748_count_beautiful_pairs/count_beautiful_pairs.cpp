#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countBeautifulPairs(vector<int> &nums) {
    int res = 0, n = nums.size();
    for (int i = 0; i < n; i++) {
      while (nums[i] >= 10) {
        nums[i] /= 10;
      }
      for (int j = i + 1; j < n; j++) {
        if (gcd(nums[i], nums[j] % 10) == 1) {
          res++;
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 5, 1, 4}, 5},
      {{11, 21, 12}, 2},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().countBeautifulPairs(nums) == ans);
  }
}