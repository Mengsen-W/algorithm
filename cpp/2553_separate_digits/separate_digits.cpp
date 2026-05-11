#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> separateDigits(vector<int>& nums) {
    vector<int> res;
    for (int i = nums.size() - 1; i >= 0; i--) {
      int x = nums[i];
      while (x > 0) {
        res.push_back(x % 10);
        x /= 10;
      }
    }
    reverse(res.begin(), res.end());
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{13, 25, 83, 77}, {1, 3, 2, 5, 8, 3, 7, 7}},
      {{7, 1, 3, 9}, {7, 1, 3, 9}},
  };

  for (auto& [nums, expected] : tests) {
    assert(Solution().separateDigits(nums) == expected);
  }
}