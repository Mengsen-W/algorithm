/*
 * @Date: 2023-01-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-20
 * @FilePath: /algorithm/cpp/1817_finding_users_active_minutes/finding_users_active_minutes.cpp
 */

#include <cassert>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findingUsersActiveMinutes(vector<vector<int>> &logs, int k) {
    unordered_map<int, unordered_set<int>> activeMinutes;
    for (auto &&log : logs) {
      int id = log[0], time = log[1];
      activeMinutes[id].emplace(time);
    }
    vector<int> answer(k);
    for (auto &&[_, minutes] : activeMinutes) {
      int activeCount = minutes.size();
      answer[activeCount - 1]++;
    }
    return answer;
  }
};

int main() {
  {
    vector<vector<int>> logs{{0, 5}, {1, 2}, {0, 2}, {0, 5}, {1, 3}};
    int k = 5;
    vector<int> ans{0, 2, 0, 0, 0};
    assert(Solution().findingUsersActiveMinutes(logs, k) == ans);
  }

  {
    vector<vector<int>> logs{{1, 1}, {2, 2}, {2, 3}};
    int k = 4;
    vector<int> ans{1, 1, 0, 0};
    assert(Solution().findingUsersActiveMinutes(logs, k) == ans);
  }
}