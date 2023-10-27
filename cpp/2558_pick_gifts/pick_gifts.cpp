/*
 * @Date: 2023-10-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-28
 * @FilePath: /algorithm/cpp/2558_pick_gifts/pick_gifts.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long pickGifts(vector<int>& gifts, int k) {
    priority_queue<int> q(gifts.begin(), gifts.end());
    while (k--) {
      int x = q.top();
      q.pop();
      q.push(int(sqrt(x)));
    }
    long long res = 0;
    while (q.size()) {
      res += q.top();
      q.pop();
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{25, 64, 9, 4, 100}, 4, 29},
      {{1, 1, 1, 1}, 4, 4},
  };

  for (auto [gifts, k, ans] : tests) {
    assert(Solution().pickGifts(gifts, k) == ans);
  }
}
