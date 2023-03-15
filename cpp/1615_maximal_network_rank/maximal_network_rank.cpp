/*
 * @Date: 2023-03-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-15
 * @FilePath: /algorithm/cpp/1615_maximal_network_rank/maximal_network_rank.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximalNetworkRank(int n, vector<vector<int>>& roads) {
    int ans = 0;
    // 创建一个nxn的矩阵，记录哪两个点有连接
    vector<vector<int>> connect(n, vector<int>(n));
    // 创建一维数组n个元素，记录每个点被连接的次数
    vector<int> count(n);
    // 统计每个顶点的连接边数
    for (auto& r : roads) {
      // 哪两个点连接将其置为1，这里因为是双向道路
      connect[r[0]][r[1]] = 1;
      connect[r[1]][r[0]] = 1;
      // 统计每个点出现的次数
      count[r[0]]++;
      count[r[1]]++;
    }
    // 遍历所有的点，得到某两个点所被连接的最大次数
    for (int i = 0; i < n; i++) {
      for (int j = i + 1; j < n; j++) {
        // 两个顶点出现的次数总和减去多算的双向道路的值，一般来说若有连接则减去1若无连接则减去0
        ans = max(ans, count[i] + count[j] - connect[i][j]);
      }
    }
    return ans;
  }
};

int main() {
  {
    int n = 4;
    vector<vector<int>> roads{{0, 1}, {0, 3}, {1, 2}, {1, 3}};
    int ans = 4;
    assert(Solution().maximalNetworkRank(n, roads) == ans);
  }
  {
    int n = 5;
    vector<vector<int>> roads{{0, 1}, {0, 3}, {1, 2}, {1, 3}, {2, 3}, {2, 4}};
    int ans = 5;
    assert(Solution().maximalNetworkRank(n, roads) == ans);
  }
  {
    int n = 8;
    vector<vector<int>> roads{{0, 1}, {1, 2}, {2, 3}, {2, 4}, {5, 6}, {5, 7}};
    int ans = 5;
    assert(Solution().maximalNetworkRank(n, roads) == ans);
  }
}