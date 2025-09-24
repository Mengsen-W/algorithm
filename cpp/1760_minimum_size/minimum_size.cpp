#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumSize(vector<int>& nums, int maxOperations) {
    int left = 1, right = *max_element(nums.begin(), nums.end());
    int ans = 0;
    while (left <= right) {
      int y = (left + right) / 2;
      long long ops = 0;
      for (int x : nums) {
        ops += (x - 1) / y;
      }
      if (ops <= maxOperations) {
        ans = y;
        right = y - 1;
      } else {
        left = y + 1;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{9}, 2, 3},
      {{2, 4, 8, 2}, 4, 2},
      {{7, 17}, 2, 7},
  };

  for (auto& [nums, maxOperations, ans] : tests) {
    assert(Solution().minimumSize(nums, maxOperations) == ans);
  }
}