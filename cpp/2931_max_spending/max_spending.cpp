#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maxSpending(vector<vector<int>>& values) {
    int m = values.size(), n = values[0].size();
    priority_queue<ti3, vector<ti3>, greater<ti3>> q;
    for (int i = 0; i < m; ++i) {
      q.emplace(values[i][n - 1], i, n - 1);
    }
    long long ans = 0;
    for (int turn = 1; turn <= m * n; ++turn) {
      auto [val, i, j] = q.top();
      q.pop();
      ans += static_cast<long long>(val) * turn;
      if (j > 0) {
        q.emplace(values[i][j - 1], i, j - 1);
      }
    }
    return ans;
  }

 private:
  using ti3 = tuple<int, int, int>;
};

int main() {
  vector<tuple<vector<vector<int>>, long long>> tests{
      {{{8, 5, 2}, {6, 4, 1}, {9, 7, 3}}, 285},
      {{{10, 8, 6, 4, 2}, {9, 7, 5, 3, 2}}, 386},
  };

  for (auto&[values, ans] : tests) {
    assert(Solution().maxSpending(values) == ans);
  }
}