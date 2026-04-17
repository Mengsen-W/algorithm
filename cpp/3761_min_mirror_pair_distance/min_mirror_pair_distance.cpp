#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
 public:
  int minMirrorPairDistance(vector<int>& nums) {
    auto reverseNum = [](int x) {
      int y = 0;
      while (x > 0) {
        y = y * 10 + x % 10;
        x /= 10;
      }
      return y;
    };

    int n = nums.size();
    unordered_map<int, int> prev;
    int ans = n + 1;
    for (int i = 0; i < n; ++i) {
      int x = nums[i];
      if (prev.count(x)) {
        ans = min(ans, i - prev[x]);
      }
      prev[reverseNum(x)] = i;
    }

    return ans == n + 1 ? -1 : ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{12, 21, 45, 33, 54}, 1},
      {{120, 21}, 1},
      {{21, 120}, -1},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().minMirrorPairDistance(nums) == ans);
  }
}
