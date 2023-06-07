/*
 * @Date: 2023-06-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-07
 * @FilePath: /algorithm/cpp/2611_mice_and_cheese/mice_and_cheese.cpp
 */

#include <cassert>
#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  int miceAndCheese(vector<int>& reward1, vector<int>& reward2, int k) {
    int ans = 0;
    int n = reward1.size();
    priority_queue<int, vector<int>, greater<int>> pq;
    for (int i = 0; i < n; i++) {
      ans += reward2[i];
      pq.emplace(reward1[i] - reward2[i]);
      if (pq.size() > k) {
        pq.pop();
      }
    }
    while (!pq.empty()) {
      ans += pq.top();
      pq.pop();
    }
    return ans;
  }
};

int main() {
  {
    vector<int> reward1{1, 1, 3, 4};
    vector<int> reward2{4, 4, 1, 1};
    int k = 2;
    int ans = 15;
    assert(Solution().miceAndCheese(reward1, reward2, k) == ans);
  }

  {
    vector<int> reward1{1, 1};
    vector<int> reward2{1, 1};
    int k = 2;
    int ans = 2;
    assert(Solution().miceAndCheese(reward1, reward2, k) == ans);
  }
}