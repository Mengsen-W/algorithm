#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxProduct(vector<int>& nums) {
    int a = nums[0], b = nums[1];
    if (a < b) {
      swap(a, b);
    }
    for (int i = 2; i < nums.size(); i++) {
      if (nums[i] > a) {
        b = a;
        a = nums[i];
      } else if (nums[i] > b) {
        b = nums[i];
      }
    }
    return (a - 1) * (b - 1);
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{3, 4, 5, 2}, 12},
      {{1, 5, 4, 5}, 16},
      {{3, 7}, 12},
  };

  for (auto [test, expected] : tests) {
    assert(Solution().maxProduct(test) == expected);
  }
}
