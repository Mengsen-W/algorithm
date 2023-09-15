/*
 * @Date: 2023-09-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-15
 * @FilePath: /algorithm/cpp/1222_queens_attackthe_king/queens_attackthe_king.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> queensAttacktheKing(vector<vector<int>>& queens, vector<int>& king) {
    unordered_set<int> queen_pos;
    for (const auto& queen : queens) {
      int x = queen[0], y = queen[1];
      queen_pos.insert(x * 8 + y);
    }

    vector<vector<int>> ans;
    for (int dx = -1; dx <= 1; ++dx) {
      for (int dy = -1; dy <= 1; ++dy) {
        if (dx == 0 && dy == 0) {
          continue;
        }
        int kx = king[0] + dx, ky = king[1] + dy;
        while (kx >= 0 && kx < 8 && ky >= 0 && ky < 8) {
          int pos = kx * 8 + ky;
          if (queen_pos.count(pos)) {
            ans.push_back({kx, ky});
            break;
          }
          kx += dx;
          ky += dy;
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<int>, vector<vector<int>>>> tests{
      {{{0, 1}, {1, 0}, {4, 0}, {0, 4}, {3, 3}, {2, 4}}, {0, 0}, {{0, 1}, {1, 0}, {3, 3}}},
      {{{0, 0}, {1, 1}, {2, 2}, {3, 4}, {3, 5}, {4, 4}, {4, 5}}, {3, 3}, {{2, 2}, {3, 4}, {4, 4}}},
      {{{5, 6}, {7, 7}, {2, 1}, {0, 7}, {1, 6}, {5, 1}, {3, 7}, {0, 3}, {4, 0}, {1, 2}, {6, 3}, {5, 0},
        {0, 4}, {2, 2}, {1, 1}, {6, 4}, {5, 4}, {0, 0}, {2, 6}, {4, 5}, {5, 2}, {1, 4}, {7, 5}, {2, 3},
        {0, 5}, {4, 2}, {1, 0}, {2, 7}, {0, 1}, {4, 6}, {6, 1}, {0, 6}, {4, 3}, {1, 7}},
       {3, 4},
       {{2, 3}, {1, 4}, {1, 6}, {3, 7}, {4, 3}, {5, 4}, {4, 5}}},
  };

  for (auto& [queens, king, expected] : tests) {
    assert(Solution().queensAttacktheKing(queens, king) == expected);
  }
}