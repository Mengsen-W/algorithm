#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  long long beautifulSubarrays(vector<int>& nums) {
    unordered_map<int, int> cnt;
    int mask = 0;
    long long ans = 0;
    cnt[0] = 1;
    for (int x : nums) {
      mask ^= x;
      ans += cnt[mask];
      cnt[mask]++;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{4, 3, 1, 2, 4}, 2},
      {{1, 10, 4}, 0},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().beautifulSubarrays(nums) == ans);
  }
}