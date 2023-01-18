/*
 * @Date: 2021-09-29 09:46:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-29 10:07:25
 */

#include <cassert>
#include <cmath>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  int findMinMoves(vector<int> &machines) {
    int tot = accumulate(machines.begin(), machines.end(), 0);
    int n = machines.size();
    if (tot % n) {
      return -1;
    }
    int avg = tot / n;
    int ans = 0, sum = 0;
    for (int num : machines) {
      num -= avg;
      sum += num;
      ans = max(ans, max(abs(sum), num));
    }
    return ans;
  }
};

int main() {
  {
    vector<int> machines{1, 0, 5};
    assert(Solution().findMinMoves(machines) == 3);
  }
  {
    vector<int> machines{0, 3, 0};
    assert(Solution().findMinMoves(machines) == 2);
  }
  {
    vector<int> machines{0, 2, 0};
    assert(Solution().findMinMoves(machines) == -1);
  }
}