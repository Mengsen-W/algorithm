/*
 * @Date: 2023-05-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-16
 * @FilePath: /algorithm/cpp/1335_min_difficulty/min_difficulty.cpp
 */

#include <cassert>
#include <stack>
#include <vector>

using namespace std;

class Solution {
 public:
  int minDifficulty(vector<int>& jobDifficulty, int d) {
    int n = jobDifficulty.size();
    if (n < d) {
      return -1;
    }
    vector<int> dp(n);
    for (int j = 0, ma = 0; j < n; ++j) {
      ma = max(ma, jobDifficulty[j]);
      dp[j] = ma;
    }
    for (int i = 1; i < d; ++i) {
      stack<pair<int, int>> st;
      vector<int> ndp(n);
      for (int j = i; j < n; ++j) {
        int mi = dp[j - 1];
        while (!st.empty() && jobDifficulty[st.top().first] < jobDifficulty[j]) {
          mi = min(mi, st.top().second);
          st.pop();
        }
        if (st.empty()) {
          ndp[j] = mi + jobDifficulty[j];
        } else {
          ndp[j] = min(ndp[st.top().first], mi + jobDifficulty[j]);
        }
        st.emplace(j, mi);
      }
      swap(dp, ndp);
    }
    return dp[n - 1];
  }
};

int main() {
  {
    vector<int> jobDifficulty{6, 5, 4, 3, 2, 1};
    int d = 2;
    int ans = 7;
    assert(Solution().minDifficulty(jobDifficulty, d) == ans);
  }

  {
    vector<int> jobDifficulty{9, 9, 9};
    int d = 4;
    int ans = -1;
    assert(Solution().minDifficulty(jobDifficulty, d) == ans);
  }

  {
    vector<int> jobDifficulty{1, 1, 1};
    int d = 3;
    int ans = 3;
    assert(Solution().minDifficulty(jobDifficulty, d) == ans);
  }

  {
    vector<int> jobDifficulty{7, 1, 7, 1, 7, 1};
    int d = 3;
    int ans = 15;
    assert(Solution().minDifficulty(jobDifficulty, d) == ans);
  }

  {
    vector<int> jobDifficulty{11, 111, 22, 222, 33, 333, 44, 444};
    int d = 6;
    int ans = 843;
    assert(Solution().minDifficulty(jobDifficulty, d) == ans);
  }
}