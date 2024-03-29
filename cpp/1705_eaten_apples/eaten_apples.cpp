/*
 * @Date: 2021-12-24 01:16:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-24 01:38:17
 */

#include <cassert>
#include <iostream>
#include <queue>
#include <vector>

using namespace std;

typedef pair<int, int> pii;

class Solution {
 public:
  int eatenApples(vector<int> apples, vector<int> days) {
    int ans = 0;
    priority_queue<pii, vector<pii>, greater<pii>> pq;
    int n = apples.size();
    int i = 0;
    while (i < n) {
      while (!pq.empty() && pq.top().first <= i) {
        pq.pop();
      }
      int rottenDay = i + days[i];
      int count = apples[i];
      if (count > 0) {
        pq.emplace(rottenDay, count);
      }
      if (!pq.empty()) {
        pii curr = pq.top();
        pq.pop();
        curr.second--;
        if (curr.second != 0) {
          pq.emplace(curr.first, curr.second);
        }
        ans++;
      }
      i++;
    }
    while (!pq.empty()) {
      while (!pq.empty() && pq.top().first <= i) {
        pq.pop();
      }
      if (pq.empty()) {
        break;
      }
      pii curr = pq.top();
      pq.pop();
      int num = min(curr.first - i, curr.second);
      ans += num;
      i += num;
    }
    return ans;
  }
};

int main() {
  assert(Solution().eatenApples({1, 2, 3, 5, 2}, {3, 2, 1, 4, 2}) == 7);
  assert(Solution().eatenApples({3, 0, 0, 0, 0, 2}, {3, 0, 0, 0, 0, 2}) == 7);
}