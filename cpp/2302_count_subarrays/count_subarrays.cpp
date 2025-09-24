#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long countSubarrays(vector<int>& nums, long long k) {
    int n = nums.size();
    long long res = 0, total = 0;
    for (int i = 0, j = 0; j < n; j++) {
      total += nums[j];
      while (i <= j && total * (j - i + 1) >= k) {
        total -= nums[i];
        i++;
      }
      res += j - i + 1;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, long long, long long>> tests{
      {{2, 1, 4, 3, 5}, 10, 6},
      {{1, 1, 1}, 5, 5},
  };

  for (auto &[nums, k, ans] : tests) {
    assert(Solution().countSubarrays(nums, k) == ans);
  }
}
