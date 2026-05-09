#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<vector<int>> rotateGrid(vector<vector<int>>& grid, int k) {
    int m = grid.size();
    int n = grid[0].size();
    int nlayer = min(m / 2, n / 2);  // 层数
    // 从左上角起逆时针枚举每一层
    for (int layer = 0; layer < nlayer; ++layer) {
      vector<int> r, c, val;                         // 每个元素的行下标，列下标与数值
      for (int i = layer; i < m - layer - 1; ++i) {  // 左
        r.push_back(i);
        c.push_back(layer);
        val.push_back(grid[i][layer]);
      }
      for (int j = layer; j < n - layer - 1; ++j) {  // 下
        r.push_back(m - layer - 1);
        c.push_back(j);
        val.push_back(grid[m - layer - 1][j]);
      }
      for (int i = m - layer - 1; i > layer; --i) {  // 右
        r.push_back(i);
        c.push_back(n - layer - 1);
        val.push_back(grid[i][n - layer - 1]);
      }
      for (int j = n - layer - 1; j > layer; --j) {  // 上
        r.push_back(layer);
        c.push_back(j);
        val.push_back(grid[layer][j]);
      }
      int total = val.size();  // 每一层的元素总数
      int kk = k % total;      // 等效轮转次数
      // 找到每个下标对应的轮转后的取值
      for (int i = 0; i < total; ++i) {
        int idx = (i + total - kk) % total;  // 轮转后取值对应的下标
        grid[r[i]][c[i]] = val[idx];
      }
    }
    return grid;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, vector<vector<int>>>> tests{
      {
          {{40, 10}, {30, 20}},
          1,
          {{10, 20}, {40, 30}},
      },
      {
          {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}, {13, 14, 15, 16}},
          2,
          {{3, 4, 8, 12}, {2, 11, 10, 16}, {1, 7, 6, 15}, {5, 9, 13, 14}},
      },
  };

  for (auto [grid, k, expected] : tests) {
    assert(Solution().rotateGrid(grid, k) == expected);
  }
}