/*
 * @Date: 2022-10-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-22
 * @FilePath: /algorithm/1235_job_scheduling/job_scheduling.cpp
 */

#include <cassert>
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
  {
    vector<int> start_time{1, 2, 3, 3};
    vector<int> end_time{3, 4, 5, 6};
    vector<int> profit{50, 10, 40, 70};
    int ans = 120;
    assert(Solution().jobScheduling(start_time, end_time, profit) == ans);
  }

  {
    vector<int> start_time{1, 2, 3, 4, 6};
    vector<int> end_time{3, 5, 10, 6, 9};
    vector<int> profit{20, 20, 100, 70, 60};
    int ans = 150;
    assert(Solution().jobScheduling(start_time, end_time, profit) == ans);
  }

  {
    vector<int> start_time{1, 1, 1};
    vector<int> end_time{2, 3, 4};
    vector<int> profit{5, 6, 4};
    int ans = 6;
    assert(Solution().jobScheduling(start_time, end_time, profit) == ans);
  }
}