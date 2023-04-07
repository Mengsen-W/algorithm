/*
 * @Date: 2023-04-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-07
 * @FilePath: /algorithm/cpp/1040_num_moves_stones_ii/num_moves_stones_ii.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> numMovesStonesII(vector<int>& stones) {
    int n = stones.size();
    sort(stones.begin(), stones.end());
    if (stones.back() - stones[0] + 1 == n) {
      return {0, 0};
    }
    int ma = max(stones[n - 2] - stones[0] + 1, stones[n - 1] - stones[1] + 1) - (n - 1);
    int mi = n;
    for (int i = 0, j = 0; i < n && j + 1 < n; ++i) {
      while (j + 1 < n && stones[j + 1] - stones[i] + 1 <= n) {
        ++j;
      }
      if (j - i + 1 == n - 1 && stones[j] - stones[i] + 1 == n - 1) {
        mi = min(mi, 2);
      } else {
        mi = min(mi, n - (j - i + 1));
      }
    }
    return {mi, ma};
  }
};

int main() {
  {
    vector<int> stones{7, 4, 9};
    vector<int> ans{1, 2};
    assert(Solution().numMovesStonesII(stones) == ans);
  }

  {
    vector<int> stones{6, 5, 4, 3, 10};
    vector<int> ans{2, 3};
    assert(Solution().numMovesStonesII(stones) == ans);
  }

  {
    vector<int> stones{100, 101, 104, 102, 103};
    vector<int> ans{0, 0};
    assert(Solution().numMovesStonesII(stones) == ans);
  }
}