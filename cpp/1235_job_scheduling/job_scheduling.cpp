/*
 * @Date: 2022-10-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-04
 * @FilePath: /algorithm/cpp/1235_job_scheduling/job_scheduling.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int jobScheduling(vector<int> &startTime, vector<int> &endTime, vector<int> &profit) {
    int n = startTime.size();
    vector<vector<int>> jobs(n);
    for (int i = 0; i < n; i++) {
      jobs[i] = {startTime[i], endTime[i], profit[i]};
    }
    sort(jobs.begin(), jobs.end(),
         [](const vector<int> &job1, const vector<int> &job2) -> bool { return job1[1] < job2[1]; });
    vector<int> dp(n + 1);
    for (int i = 1; i <= n; i++) {
      int k = upper_bound(jobs.begin(), jobs.begin() + i - 1, jobs[i - 1][0],
                          [&](int st, const vector<int> &job) -> bool { return st < job[1]; }) -
              jobs.begin();
      dp[i] = max(dp[i - 1], dp[k] + jobs[i - 1][2]);
    }
    return dp[n];
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, vector<int>, int>> tests{
      {{1, 2, 3, 3}, {3, 4, 5, 6}, {50, 10, 40, 70}, 120},
      {{1, 2, 3, 4, 6}, {3, 5, 10, 6, 9}, {20, 20, 100, 70, 60}, 150},
      {{1, 1, 1}, {2, 3, 4}, {5, 6, 4}, 6},
  };

  for (auto &[startTime, endTime, profit, ans] : tests) {
    assert(Solution().jobScheduling(startTime, endTime, profit) == ans);
  }
}