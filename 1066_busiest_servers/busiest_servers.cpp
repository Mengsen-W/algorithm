/*
 * @Date: 2022-03-29 23:53:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-30 00:07:54
 * @FilePath: /algorithm/1066_busiest_servers/busiest_servers.cpp
 */

#include <algorithm>
#include <cassert>
#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> busiestServers(int k, vector<int> &arrival, vector<int> &load) {
    priority_queue<int, vector<int>, greater<int>> available;
    for (int i = 0; i < k; i++) {
      available.push(i);
    }
    priority_queue<pair<int, int>, vector<pair<int, int>>,
                   greater<pair<int, int>>>
        busy;
    vector<int> requests(k, 0);
    for (int i = 0; i < arrival.size(); i++) {
      while (!busy.empty() && busy.top().first <= arrival[i]) {
        auto [_, id] = busy.top();
        busy.pop();
        available.push(i + ((id - i) % k + k) % k);
        // 保证得到的是一个不小于 i 的且与 id 同余的数
      }
      if (available.empty()) {
        continue;
      }
      int id = available.top() % k;
      available.pop();
      requests[id]++;
      busy.push({arrival[i] + load[i], id});
    }
    int maxRequest = *max_element(requests.begin(), requests.end());
    vector<int> ret;
    for (int i = 0; i < k; i++) {
      if (requests[i] == maxRequest) {
        ret.push_back(i);
      }
    }
    return ret;
  }
};

int main() {
  {
    int k = 3;
    vector<int> arrival{1, 2, 3, 4, 5};
    vector<int> load{5, 2, 3, 3, 3};
    vector<int> ans{1};
    assert(Solution().busiestServers(k, arrival, load) == ans);
  }

  {
    int k = 3;
    vector<int> arrival{1, 2, 3, 4};
    vector<int> load{1, 2, 1, 2};
    vector<int> ans{0};
    assert(Solution().busiestServers(k, arrival, load) == ans);
  }

  {
    int k = 3;
    vector<int> arrival{1, 2, 3};
    vector<int> load{10, 12, 11};
    vector<int> ans{0, 1, 2};
    assert(Solution().busiestServers(k, arrival, load) == ans);
  }

  {
    int k = 3;
    vector<int> arrival{1, 2, 3, 4, 8, 9, 10};
    vector<int> load{5, 2, 10, 3, 1, 2, 2};
    vector<int> ans{1};
    assert(Solution().busiestServers(k, arrival, load) == ans);
  }

  {
    int k = 3;
    vector<int> arrival{1};
    vector<int> load{1};
    vector<int> ans{0};
    assert(Solution().busiestServers(k, arrival, load) == ans);
  }
}