#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool canAliceWin(vector<int>& nums) {
    int single_digit_sum = 0;
    int double_digit_sum = 0;
    for (auto num : nums) {
      if (num < 10) {
        single_digit_sum += num;
      } else {
        double_digit_sum += num;
      }
    }
    return single_digit_sum != double_digit_sum;
  }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{1, 2, 3, 4, 10}, false},
      {{1, 2, 3, 4, 5, 14}, true},
      {{5, 5, 5, 25}, true},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().canAliceWin(nums) == ans);
  }
}