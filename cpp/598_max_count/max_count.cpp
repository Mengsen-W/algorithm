#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxCount(int m, int n, vector<vector<int>>& ops) {
    int mina = m, minb = n;
    for (const auto& op : ops) {
      mina = min(mina, op[0]);
      minb = min(minb, op[1]);
    }
    return mina * minb;
  }
};

int main() {
  vector<tuple<int, int, vector<vector<int>>, int>> tests{
      {3, 3, {{2, 2}, {3, 3}}, 4},
      {3, 3, {{2, 2}, {3, 3}, {3, 3}, {3, 3}, {2, 2}, {3, 3}, {3, 3}, {3, 3}, {2, 2}, {3, 3}, {3, 3}, {3, 3}}, 4},
  };

  for (auto& [m, n, ops, ans] : tests) {
    assert(Solution().maxCount(m, n, ops) == ans);
  }
}