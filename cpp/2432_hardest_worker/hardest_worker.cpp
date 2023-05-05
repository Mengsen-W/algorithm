/*
 * @Date: 2023-05-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-05
 * @FilePath: /algorithm/cpp/2432_hardest_worker/hardest_worker.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int hardestWorker(int n, vector<vector<int>>& logs) {
    int ans = logs[0][0], maxcost = logs[0][1];
    for (int i = 1; i < logs.size(); ++i) {
      int idx = logs[i][0];
      int cost = logs[i][1] - logs[i - 1][1];
      if (cost > maxcost || (cost == maxcost && idx < ans)) {
        ans = idx;
        maxcost = cost;
      }
    }
    return ans;
  }
};

int main() {
  {
    int n = 10;
    vector<vector<int>> logs = {{0, 3}, {2, 5}, {0, 9}, {1, 15}};
    int ans = 1;
    assert(Solution().hardestWorker(n, logs) == ans);
  }

  {
    int n = 26;
    vector<vector<int>> logs = {{1, 1}, {3, 7}, {2, 12}, {7, 17}};
    int ans = 3;
    assert(Solution().hardestWorker(n, logs) == ans);
  }

  {
    int n = 2;
    vector<vector<int>> logs = {{0, 10}, {1, 20}};
    int ans = 0;
    assert(Solution().hardestWorker(n, logs) == ans);
  }
}