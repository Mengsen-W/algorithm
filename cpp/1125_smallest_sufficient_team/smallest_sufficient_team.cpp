/*
 * @Date: 2023-04-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-08
 * @FilePath: /algorithm/cpp/1125_smallest_sufficient_team/smallest_sufficient_team.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> smallestSufficientTeam(vector<string>& req_skills, vector<vector<string>>& people) {
    int n = req_skills.size(), m = people.size();
    unordered_map<string, int> skill_index;
    for (int i = 0; i < n; ++i) {
      skill_index[req_skills[i]] = i;
    }
    vector<int> dp(1 << n, m);
    dp[0] = 0;
    vector<int> prev_skill(1 << n, 0);
    vector<int> prev_people(1 << n, 0);
    for (int i = 0; i < m; ++i) {
      int cur_skill = 0;
      for (string& skill : people[i]) {
        cur_skill |= 1 << skill_index[skill];
      }
      for (int prev = 0; prev < (1 << n); prev++) {
        int comb = prev | cur_skill;
        if (dp[comb] > dp[prev] + 1) {
          dp[comb] = dp[prev] + 1;
          prev_skill[comb] = prev;
          prev_people[comb] = i;
        }
      }
    }
    vector<int> res;
    int i = (1 << n) - 1;
    while (i > 0) {
      res.push_back(prev_people[i]);
      i = prev_skill[i];
    }
    return res;
  }
};

int main() {
  {
    vector<string> req_skills{"java", "nodejs", "reactjs"};
    vector<vector<string>> people{{"java"}, {"nodejs"}, {"nodejs", "reactjs"}};
    vector<int> ans{2, 0};
    assert(Solution().smallestSufficientTeam(req_skills, people) == ans);
  }

  {
    vector<string> req_skills{"algorithms", "math", "java", "reactjs", "csharp", "aws"};
    vector<vector<string>> people{{"algorithms", "math", "java"},
                                  {"algorithms", "math", "reactjs"},
                                  {"java", "csharp", "aws"},
                                  {"reactjs", "csharp"},
                                  {"csharp", "math"},
                                  {"aws", "java"}};
    vector<int> ans{2, 1};
    assert(Solution().smallestSufficientTeam(req_skills, people) == ans);
  }
}