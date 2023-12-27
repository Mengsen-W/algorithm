/*
 * @Date: 2023-12-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-26
 * @FilePath: /algorithm/cpp/1349_max_students/max_students.cpp
 */

#include <cassert>
#include <functional>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxStudents(vector<vector<char>>& seats) {
    int m = seats.size(), n = seats[0].size();
    unordered_map<int, int> memo;

    auto isSingleRowCompliant = [&](int status, int row) -> bool {
      for (int j = 0; j < n; j++) {
        if ((status >> j) & 1) {
          if (seats[row][j] == '#') {
            return false;
          }
          if (j > 0 && ((status >> (j - 1)) & 1)) {
            return false;
          }
        }
      }
      return true;
    };

    auto isCrossRowsCompliant = [&](int status, int upperRowStatus) -> bool {
      for (int j = 0; j < n; j++) {
        if ((status >> j) & 1) {
          if (j > 0 && ((upperRowStatus >> (j - 1)) & 1)) {
            return false;
          }
          if (j < n - 1 && ((upperRowStatus >> (j + 1)) & 1)) {
            return false;
          }
        }
      }
      return true;
    };

    function<int(int, int)> dp = [&](int row, int status) -> int {
      int key = (row << n) + status;
      if (!memo.count(key)) {
        if (!isSingleRowCompliant(status, row)) {
          memo[key] = INT_MIN;
          return INT_MIN;
        }
        int students = __builtin_popcount(status);
        if (row == 0) {
          memo[key] = students;
          return students;
        }
        int mx = 0;
        for (int upperRowStatus = 0; upperRowStatus < 1 << n; upperRowStatus++) {
          if (isCrossRowsCompliant(status, upperRowStatus)) {
            mx = max(mx, dp(row - 1, upperRowStatus));
          }
        }
        memo[key] = students + mx;
      }
      return memo[key];
    };

    int mx = 0;
    for (int i = 0; i < (1 << n); i++) {
      mx = max(mx, dp(m - 1, i));
    }
    return mx;
  }
};

int main() {
  vector<tuple<vector<vector<char>>, int>> tests{
      {{{'#', '.', '#', '#', '.', '#'}, {'.', '#', '#', '#', '#', '.'}, {'#', '.', '#', '#', '.', '#'}}, 4},
      {{{'.', '#'}, {'#', '#'}, {'#', '.'}, {'#', '#'}, {'.', '#'}}, 3},
      {{{'#', '.', '.', '.', '#'},
        {'.', '#', '.', '#', '.'},
        {'.', '.', '#', '.', '.'},
        {'.', '#', '.', '#', '.'},
        {'#', '.', '.', '.', '#'}},
       10},
  };

  for (auto& [seats, ans] : tests) {
    assert(Solution().maxStudents(seats) == ans);
  }
}