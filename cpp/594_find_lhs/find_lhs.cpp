#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int findLHS(vector<int> nums) {
    unordered_map<int, int> cnt;
    int res = 0;
    for (int num : nums) cnt[num]++;

    for (auto [key, val] : cnt) {
      if (cnt.count(key + 1)) res = max(res, val + cnt[key + 1]);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{0, 3, 2, 2, 5, 2, 3, 7}, 5},
      {{0, 2, 3, 4}, 2},
      {{0, 1, 1, 1}, 0},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().findLHS(nums) == ans);
  }
}