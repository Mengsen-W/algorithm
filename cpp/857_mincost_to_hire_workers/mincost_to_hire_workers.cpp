/*
 * @Date: 2022-09-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-11
 * @FilePath: /algorithm/857_mincost_to_hire_workers/mincost_to_hire_workers.cpp
 */

#include <assert.h>

#include <algorithm>
#include <numeric>
#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  double mincostToHireWorkers(vector<int>& quality, vector<int>& wage, int k) {
    int n = quality.size();
    vector<int> h(n, 0);
    iota(h.begin(), h.end(), 0);
    sort(h.begin(), h.end(), [&](int& a, int& b) { return quality[a] * wage[b] > quality[b] * wage[a]; });
    double res = 1e9;
    double totalq = 0.0;
    priority_queue<int, vector<int>, less<int>> q;
    for (int i = 0; i < k - 1; i++) {
      totalq += quality[h[i]];
      q.push(quality[h[i]]);
    }
    for (int i = k - 1; i < n; i++) {
      int idx = h[i];
      totalq += quality[idx];
      q.push(quality[idx]);
      double totalc = ((double)wage[idx] / quality[idx]) * totalq;
      res = min(res, totalc);
      totalq -= q.top();
      q.pop();
    }
    return res;
  }
};

int main() {
  {
    vector<int> quality{10, 20, 5};
    vector<int> wage{70, 50, 30};
    int k = 2;
    double ans{105.00000};
    assert(Solution().mincostToHireWorkers(quality, wage, k) == ans);
  }

  {
    vector<int> quality{3, 1, 10, 10, 1};
    vector<int> wage{4, 8, 2, 2, 7};
    int k = 3;
    double ans = Solution().mincostToHireWorkers(quality, wage, k);
    assert(Solution().mincostToHireWorkers(quality, wage, k) == ans);
  }
}