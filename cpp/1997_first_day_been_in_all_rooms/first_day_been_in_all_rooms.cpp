/*
 * @Date: 2024-03-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-28
 * @FilePath: /algorithm/cpp/1997_first_day_been_in_all_rooms/first_day_been_in_all_rooms.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int firstDayBeenInAllRooms(vector<int>& nextVisit) {
    int mod = 1e9 + 7;
    int len = nextVisit.size();
    vector<int> dp(len);

    dp[0] = 2;  // 初始化原地待一天 + 访问下一个房间一天
    for (int i = 1; i < len; i++) {
      int to = nextVisit[i];
      dp[i] = 2 + dp[i - 1];

      if (to != 0) {
        dp[i] = (dp[i] - dp[to - 1] + mod) % mod;  // 避免负数
      }
      dp[i] = (dp[i] + dp[i - 1]) % mod;
    }
    return dp[len - 2];  // 题目保证n >= 2
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{0, 0}, 2},
      {{0, 0, 2}, 6},
      {{0, 1, 2, 0}, 6},
  };

  for (auto& [nextVisit, ans] : tests) {
    assert(Solution().firstDayBeenInAllRooms(nextVisit) == ans);
  }
}