/*
 * @Date: 2021-09-08 09:31:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-08 10:42:32
 */

#include <cassert>
#include <functional>
#include <queue>
#include <vector>

using namespace std;

typedef pair<int, int> pii;

class Solution {
 public:
  int findMaximizedCapital(int k, int w, vector<int>& profits,
                           vector<int>& capital) {
    int n = profits.size();
    int curr = 0;
    priority_queue<int, vector<int>, less<int>> pq;
    vector<pii> arr;

    for (int i = 0; i < n; ++i) {
      arr.push_back({capital[i], profits[i]});
    }
    sort(arr.begin(), arr.end());
    for (int i = 0; i < k; ++i) {
      while (curr < n && arr[curr].first <= w) {
        pq.push(arr[curr].second);
        curr++;
      }
      if (!pq.empty()) {
        w += pq.top();
        pq.pop();
      } else {
        break;
      }
    }

    return w;
  }
};

int main() {
  {
    int k = 2;
    int w = 0;
    vector<int> profits{1, 2, 3};
    vector<int> capital{0, 1, 1};
    int ans = 4;
    assert(Solution().findMaximizedCapital(k, w, profits, capital) == ans);
  }
  {
    int k = 3;
    int w = 0;
    vector<int> profits{1, 2, 3};
    vector<int> capital{0, 1, 2};
    int ans = 6;
    assert(Solution().findMaximizedCapital(k, w, profits, capital) == ans);
  }
}