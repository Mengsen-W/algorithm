#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  using ll = long long;
  long long countBadPairs(vector<int>& nums) {
    unordered_map<int, int> mp;
    ll res = 0;
    for (int i = 0; i < nums.size(); i++) {
      res += i - mp[nums[i] - i];
      mp[nums[i] - i]++;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{4, 1, 3, 3}, 5},
      {{1, 2, 3, 4, 5}, 0},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().countBadPairs(nums) == ans);
  }
}