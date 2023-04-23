/*
 * @Date: 2023-04-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-23
 * @FilePath: /algorithm/cpp/1105_min_height_shelves/min_height_shelves.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minHeightShelves(vector<vector<int>>& books, int shelfWidth) {
    int n = books.size();
    vector<int> dp(n + 1, 1000000);
    dp[0] = 0;
    for (int i = 0; i < n; ++i) {
      int maxHeight = 0, curWidth = 0;
      for (int j = i; j >= 0; --j) {
        curWidth += books[j][0];
        if (curWidth > shelfWidth) {
          break;
        }
        maxHeight = max(maxHeight, books[j][1]);
        dp[i + 1] = min(dp[i + 1], dp[j] + maxHeight);
      }
    }
    return dp[n];
  }
};

int main() {
  {
    vector<vector<int>> books{{1, 1}, {2, 3}, {2, 3}, {1, 1}, {1, 1}, {1, 1}, {1, 2}};
    int shelfWidth = 4;
    int ans = 6;
    assert(Solution().minHeightShelves(books, shelfWidth) == ans);
  }

  {
    vector<vector<int>> books{{1, 3}, {2, 4}, {3, 2}};
    int shelfWidth = 6;
    int ans = 4;
    assert(Solution().minHeightShelves(books, shelfWidth) == ans);
  }
}
