/*
 * @Date: 2023-03-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-22
 * @FilePath: /algorithm/cpp/1626_best_team_score/best_team_score.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int bestTeamScore(vector<int>& scores, vector<int>& ages) {
    int n = scores.size();
    vector<pair<int, int>> people;
    for (int i = 0; i < n; ++i) {
      people.push_back({scores[i], ages[i]});
    }
    sort(people.begin(), people.end());
    vector<int> dp(n, 0);
    int res = 0;
    for (int i = 0; i < n; ++i) {
      for (int j = i - 1; j >= 0; --j) {
        if (people[j].second <= people[i].second) {
          dp[i] = max(dp[i], dp[j]);
        }
      }
      dp[i] += people[i].first;
      res = max(res, dp[i]);
    }
    return res;
  }
};

int main() {
  {
    vector<int> scores{1, 3, 5, 10, 15};
    vector<int> ages{1, 2, 3, 4, 5};
    int ans = 34;
    assert(Solution().bestTeamScore(scores, ages) == ans);
  }

  {
    vector<int> scores{4, 5, 6, 5};
    vector<int> ages{2, 1, 2, 1};
    int ans = 16;
    assert(Solution().bestTeamScore(scores, ages) == ans);
  }

  {
    vector<int> scores{1, 2, 3, 5};
    vector<int> ages{8, 9, 10, 1};
    int ans = 6;
    assert(Solution().bestTeamScore(scores, ages) == ans);
  }
}
