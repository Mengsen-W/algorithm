/*
 * @Date: 2022-12-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-05
 * @FilePath: /algorithm/1687_box_delivering/box_delivering.cpp
 */

#include <cassert>
#include <deque>
#include <vector>

using namespace std;

class Solution {
 public:
  int boxDelivering(vector<vector<int>>& boxes, int portsCount, int maxBoxes, int maxWeight) {
    int n = boxes.size();
    vector<int> p(n + 1), w(n + 1), neg(n + 1);
    vector<long long> W(n + 1);
    for (int i = 1; i <= n; ++i) {
      p[i] = boxes[i - 1][0];
      w[i] = boxes[i - 1][1];
      if (i > 1) {
        neg[i] = neg[i - 1] + (p[i - 1] != p[i]);
      }
      W[i] = W[i - 1] + w[i];
    }

    deque<int> opt = {0};
    vector<int> f(n + 1), g(n + 1);

    for (int i = 1; i <= n; ++i) {
      while (i - opt.front() > maxBoxes || W[i] - W[opt.front()] > maxWeight) {
        opt.pop_front();
      }

      f[i] = g[opt.front()] + neg[i] + 2;

      if (i != n) {
        g[i] = f[i] - neg[i + 1];
        while (!opt.empty() && g[i] <= g[opt.back()]) {
          opt.pop_back();
        }
        opt.push_back(i);
      }
    }

    return f[n];
  }
};

int main() {
  {
    vector<vector<int>> boxes{{1, 1}, {2, 1}, {1, 1}};
    int portsCount = 2;
    int maxBoxes = 3;
    int maxWeight = 3;
    int ans = 4;
    assert(Solution().boxDelivering(boxes, portsCount, maxBoxes, maxWeight) == ans);
  }

  {
    vector<vector<int>> boxes{{1, 2}, {3, 3}, {3, 1}, {3, 1}, {2, 4}};
    int portsCount = 3;
    int maxBoxes = 3;
    int maxWeight = 6;
    int ans = 6;
    assert(Solution().boxDelivering(boxes, portsCount, maxBoxes, maxWeight) == ans);
  }

  {
    vector<vector<int>> boxes{{1, 4}, {1, 2}, {2, 1}, {2, 1}, {3, 2}, {3, 4}};
    int portsCount = 3;
    int maxBoxes = 6;
    int maxWeight = 7;
    int ans = 6;
    assert(Solution().boxDelivering(boxes, portsCount, maxBoxes, maxWeight) == ans);
  }

  {
    vector<vector<int>> boxes{{2, 4}, {2, 5}, {3, 1}, {3, 2}, {3, 7}, {3, 1}, {4, 4}, {1, 3}, {5, 2}};
    int portsCount = 5;
    int maxBoxes = 5;
    int maxWeight = 7;
    int ans = 14;
    assert(Solution().boxDelivering(boxes, portsCount, maxBoxes, maxWeight) == ans);
  }
}