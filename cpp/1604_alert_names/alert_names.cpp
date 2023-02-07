/*
 * @Date: 2023-02-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-07
 * @FilePath: /algorithm/cpp/1604_alert_names/alert_names.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> alertNames(vector<string>& keyName, vector<string>& keyTime) {
    unordered_map<string, vector<int>> timeMap;
    int n = keyName.size();
    for (int i = 0; i < n; i++) {
      string name = keyName[i];
      string time = keyTime[i];
      int hour = (time[0] - '0') * 10 + (time[1] - '0');
      int minute = (time[3] - '0') * 10 + (time[4] - '0');
      timeMap[name].emplace_back(hour * 60 + minute);
    }
    vector<string> res;
    for (auto& [name, list] : timeMap) {
      sort(list.begin(), list.end());
      int size = list.size();
      for (int i = 2; i < size; i++) {
        int time1 = list[i - 2], time2 = list[i];
        int difference = time2 - time1;
        if (difference <= 60) {
          res.emplace_back(name);
          break;
        }
      }
    }
    sort(res.begin(), res.end());
    return res;
  }
};

int main() {
  {
    vector<string> keyName{"daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"};
    vector<string> keyTime{"10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"};
    vector<string> ans{"daniel"};
    assert(Solution().alertNames(keyName, keyTime) == ans);
  }

  {
    vector<string> keyName{"alice", "alice", "alice", "bob", "bob", "bob", "bob"};
    vector<string> keyTime{"12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"};
    vector<string> ans{"bob"};
    assert(Solution().alertNames(keyName, keyTime) == ans);
  }

  {
    vector<string> keyName{"john", "john", "john"};
    vector<string> keyTime{"23:58", "23:59", "00:01"};
    vector<string> ans{};
    assert(Solution().alertNames(keyName, keyTime) == ans);
  }

  {
    vector<string> keyName{"leslie", "leslie", "leslie", "clare", "clare", "clare", "clare"};
    vector<string> keyTime{"13:00", "13:20", "14:00", "18:00", "18:51", "19:30", "19:49"};
    vector<string> ans{"clare", "leslie"};
    assert(Solution().alertNames(keyName, keyTime) == ans);
  }
}
