/*
 * @Date: 2023-05-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-01
 * @FilePath: /algorithm/cpp/1376_num_of_minutes/num_of_minutes.cpp
 */

#include <cassert>
#include <functional>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int numOfMinutes(int n, int headID, vector<int>& manager, vector<int>& informTime) {
    unordered_map<int, int> memo;
    function<int(int)> dfs = [&](int cur) -> int {
      if (cur == headID) {
        return 0;
      }
      if (memo.count(cur)) {
        return memo[cur];
      }
      int res = dfs(manager[cur]) + informTime[manager[cur]];
      memo[cur] = res;
      return res;
    };
    int res = 0;
    for (int i = 0; i < n; i++) {
      res = max(res, dfs(i));
    }
    return res;
  }
};

int main() {
  {
    int n = 1;
    int headID = 0;
    vector<int> manager{-1};
    vector<int> informTime{0};
    int ans = 0;
    assert(Solution().numOfMinutes(n, headID, manager, informTime) == ans);
  }

  {
    int n = 5;
    int headID = 2;
    vector<int> manager{2,2,-1,2,2,2};
    vector<int> informTime{0,0,1,0,0,0};
    int ans = 1;
    assert(Solution().numOfMinutes(n, headID, manager, informTime) == ans);
  }
}
