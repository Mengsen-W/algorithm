#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countPartitions(vector<int>& nums) {
    int totalSum = 0;
    for (int x : nums) {
      totalSum += x;
    }
    return totalSum % 2 == 0 ? nums.size() - 1 : 0;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{10, 10, 3, 7, 6}, 4},
      {{1, 2, 2}, 0},
      {{2, 4, 6, 8}, 3},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().countPartitions(nums) == ans);
  }
}