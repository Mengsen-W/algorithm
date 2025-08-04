#include <cassert>
#include <cstdlib>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxTotalFruits(vector<vector<int>>& fruits, int startPos, int k) {
    int left = 0;
    int right = 0;
    int n = fruits.size();
    int sum = 0;
    int ans = 0;

    auto step = [&](int left, int right) -> int {
      if (fruits[right][0] <= startPos) {
        return startPos - fruits[left][0];
      } else if (fruits[left][0] >= startPos) {
        return fruits[right][0] - startPos;
      } else {
        return min(abs(startPos - fruits[right][0]), abs(startPos - fruits[left][0])) + fruits[right][0] -
               fruits[left][0];
      }
    };
    // 每次固定住窗口右边界
    while (right < n) {
      sum += fruits[right][1];
      // 移动左边界
      while (left <= right && step(left, right) > k) {
        sum -= fruits[left][1];
        left++;
      }
      ans = max(ans, sum);
      right++;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int, int>> tests{
      {{{2, 8}, {6, 3}, {8, 6}}, 5, 4, 9},
      {{{0, 9}, {4, 1}, {5, 7}, {6, 2}, {7, 4}, {10, 9}}, 5, 4, 14},
      {{{0, 3}, {6, 4}, {8, 5}}, 3, 2, 0},
  };

  for (auto& [fruits, startPos, k, ans] : tests) {
    assert(Solution().maxTotalFruits(fruits, startPos, k) == ans);
  }
}
