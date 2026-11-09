#include <algorithm>
#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> busiestServers(int k, vector<int> &arrival, vector<int> &load) {
        priority_queue<int, vector<int>, greater<int>> available;
        for (int i = 0; i < k; i++) {
            available.push(i);
        }
        priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> busy;
        vector<int> requests(k, 0);
        for (int i = 0; i < arrival.size(); i++) {
            while (!busy.empty() && busy.top().first <= arrival[i]) {
                auto[_, id] = busy.top();
                busy.pop();
                available.push(i + ((id - i) % k + k) % k); // 保证得到的是一个不小于 i 的且与 id 同余的数
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
  vector<tuple<int, vector<int>, vector<int>, vector<int>>> tests{
      {3, {1, 2, 3, 4, 5}, {5, 2, 3, 3, 3}, {1}},
      {3, {1, 2, 3, 4}, {1, 2, 1, 2}, {0}},
      {3, {1, 2, 3}, {10, 12, 11}, {0, 1, 2}},
      {3, {1, 2, 3, 4, 8, 9, 10}, {5, 2, 10, 3, 1, 2, 2}, {1}},
      {1, {1}, {1}, {0}},
  };

  for (auto& [k, arrival, load, ans] : tests) {
    assert(Solution().busiestServers(k, arrival, load) == ans);
  }
  return 0;
}