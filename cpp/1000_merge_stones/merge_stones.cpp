/*
 * @Date: 2023-04-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-04
 * @FilePath: /algorithm/cpp/1000_merge_stones/merge_stones.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
  static constexpr int inf = 0x3f3f3f3f;

 public:
  int mergeStones(vector<int>& stones, int k) {
    int n = stones.size();
    if ((n - 1) % (k - 1) != 0) {
      return -1;
    }

    vector d(n, vector<int>(n, inf));
    vector<int> sum(n, 0);

    for (int i = 0, s = 0; i < n; i++) {
      d[i][i] = 0;
      s += stones[i];
      sum[i] = s;
    }

    for (int len = 2; len <= n; len++) {
      for (int l = 0; l < n && l + len - 1 < n; l++) {
        int r = l + len - 1;
        for (int p = l; p < r; p += k - 1) {
          d[l][r] = min(d[l][r], d[l][p] + d[p + 1][r]);
        }
        if ((r - l) % (k - 1) == 0) {
          d[l][r] += sum[r] - (l == 0 ? 0 : sum[l - 1]);
        }
      }
    }
    return d[0][n - 1];
  }
};

int main() {
  {
    vector<int> stones{3, 2, 4, 1};
    int k = 2;
    int ans = 20;
    assert(Solution().mergeStones(stones, k) == ans);
  }

  {
    vector<int> stones{3, 2, 4, 1};
    int k = 3;
    int ans = -1;
    assert(Solution().mergeStones(stones, k) == ans);
  }

  {
    vector<int> stones{3,5,1,2,6};
    int k = 3;
    int ans = 25;
    assert(Solution().mergeStones(stones, k) == ans);
  }
}