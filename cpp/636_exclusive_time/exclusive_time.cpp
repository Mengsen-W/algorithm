/*
 * @Date: 2022-08-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-07
 * @FilePath: /algorithm/636_exclusive_time/exclusive_time.cpp
 */

#include <cassert>
#include <stack>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> exclusiveTime(int n, vector<string>& logs) {
    stack<pair<int, int>> st;  // {idx, 开始运行的时间}
    vector<int> res(n, 0);
    for (auto& log : logs) {
      char type[10];
      int idx, timestamp;
      sscanf(log.c_str(), "%d:%[^:]:%d", &idx, type, &timestamp);
      if (type[0] == 's') {
        if (!st.empty()) {
          res[st.top().first] += timestamp - st.top().second;
          st.top().second = timestamp;
        }
        st.emplace(idx, timestamp);
      } else {
        auto t = st.top();
        st.pop();
        res[t.first] += timestamp - t.second + 1;
        if (!st.empty()) {
          st.top().second = timestamp + 1;
        }
      }
    }
    return res;
  }
};

int main() {
  {
    int n = 2;
    vector<string> logs{"0:start:0", "1:start:2", "1:end:5", "0:end:6"};
    vector<int> ans{3, 4};
    assert(Solution().exclusiveTime(n, logs) == ans);
  }
  {
    int n = 1;
    vector<string> logs{"0:start:0", "0:start:2", "0:end:5", "0:start:6", "0:end:6", "0:end:7"};
    vector<int> ans{8};
    assert(Solution().exclusiveTime(n, logs) == ans);
  }
  {
    int n = 2;
    vector<string> logs{"0:start:0", "0:start:2", "0:end:5", "1:start:6", "1:end:6", "0:end:7"};
    vector<int> ans{7, 1};
    assert(Solution().exclusiveTime(n, logs) == ans);
  }
  {
    int n = 2;
    vector<string> logs{"0:start:0", "0:start:2", "0:end:5", "1:start:7", "1:end:7", "0:end:8"};
    vector<int> ans{8, 1};
    assert(Solution().exclusiveTime(n, logs) == ans);
  }
  {
    int n = 1;
    vector<string> logs{"0:start:0", "0:end:0"};
    vector<int> ans{1};
    assert(Solution().exclusiveTime(n, logs) == ans);
  }
}
