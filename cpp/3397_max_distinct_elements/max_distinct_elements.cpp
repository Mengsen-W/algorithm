#include <algorithm>
#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxDistinctElements(vector<int>& nums, int k) {
    sort(nums.begin(), nums.end());
    int cnt = 0, prev = INT_MIN;
    for (int num : nums) {
      int curr = min(max(num - k, prev + 1), num + k);
      if (curr > prev) {
        cnt++;
        prev = curr;
      }
    }
    return cnt;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2, 2, 3, 3, 4}, 2, 6},
      {{4, 4, 4, 4}, 1, 3},
  };

  for (auto& [nums, k, expect] : tests) {
    assert(Solution().maxDistinctElements(nums, k) == expect);
  }
}