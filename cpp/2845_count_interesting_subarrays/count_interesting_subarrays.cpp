#include <cassert>
#include <tuple>
#include <vector>
#include "unordered_map"

using namespace std;

class Solution {
 public:
  long long countInterestingSubarrays(vector<int>& nums, int modulo, int k) {
    int n = nums.size();
    unordered_map<int, int> cnt;
    long long res = 0;
    int prefix = 0;
    cnt[0] = 1;
    for (int i = 0; i < n; i++) {
      prefix += nums[i] % modulo == k;
      res += cnt[(prefix - k + modulo) % modulo];
      cnt[prefix % modulo]++;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, long long>> tests{
      {{3, 2, 4}, 2, 1, 3},
      {{3, 1, 9, 6}, 3, 0, 2},
  };

  for (auto& [nums, modulo, k, ans] : tests) {
    assert(Solution().countInterestingSubarrays(nums, modulo, k) == ans);
  }
}