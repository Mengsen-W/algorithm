#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumSubarrayLength(vector<int>& nums, int k) {
    int n = nums.size();
    vector<int> bits(30);
    auto calc = [](vector<int>& bits) -> int {
      int ans = 0;
      for (int i = 0; i < bits.size(); i++) {
        if (bits[i] > 0) {
          ans |= 1 << i;
        }
      }
      return ans;
    };

    int res = INT_MAX;
    for (int left = 0, right = 0; right < n; right++) {
      for (int i = 0; i < 30; i++) {
        bits[i] += (nums[right] >> i) & 1;
      }
      while (left <= right && calc(bits) >= k) {
        res = min(res, right - left + 1);
        for (int i = 0; i < 30; i++) {
          bits[i] -= (nums[left] >> i) & 1;
        }
        left++;
      }
    }

    return res == INT_MAX ? -1 : res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2, 3}, 2, 1},
      {{2, 1, 8}, 10, 3},
      {{1, 2}, 0, 1},
  };

  for (auto &[nums, k, ans] : tests ) {
    assert(Solution().minimumSubarrayLength(nums, k) == ans);
  }
}