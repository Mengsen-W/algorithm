#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int climbStairs(int n, vector<int>& costs) {
    int a = 0, b = 0, c = 0;
    for (int i = 0; i < n; ++i) {
      int nxt = min({a + 9, b + 4, c + 1}) + costs[i];
      a = b;
      b = c;
      c = nxt;
    }
    return c;
  }
};

int main() {
  vector<tuple<int, vector<int>, int>> tests{
      {4, {1, 2, 3, 4}, 13},
      {4, {5, 1, 6, 2}, 11},
      {3, {9, 8, 3}, 12},
  };

  for (auto& [n, costs, expected] : tests) {
    assert(Solution().climbStairs(n, costs) == expected);
  }
}