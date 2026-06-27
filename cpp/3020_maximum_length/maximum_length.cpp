#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
 public:
  int maximumLength(vector<int>& nums) {
    unordered_map<long long, int> cnt;
    for (int num : nums) {
      cnt[num]++;
    }
    int ans = 0;
    // ans 至少是 1 的数量，向下取奇数
    if (cnt[1] % 2 == 0) {
      ans = cnt[1] - 1;
    } else {
      ans = cnt[1];
    }
    cnt.erase(1);
    for (auto& [num, _] : cnt) {
      int res = 0;
      long long x = num;
      for (; cnt.contains(x) && cnt[x] > 1; x *= x) {
        res += 2;
      }
      ans = max(ans, res + (cnt.contains(x) ? 1 : -1));
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{5, 4, 1, 2, 2}, 3},
      {{1, 3, 2, 4}, 1},
  };

  for (auto& [nums, expect] : tests) {
    assert(Solution().maximumLength(nums) == expect);
  }
}