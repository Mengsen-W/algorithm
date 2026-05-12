#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int minimumEffort(vector<vector<int>> &tasks) {
    sort(tasks.begin(), tasks.end(), [&](vector<int> &a, vector<int> &b) { return a[1] - a[0] > b[1] - b[0]; });
    int ans = 0;
    int remain = 0;
    for (auto task : tasks) {
      ans += remain > task[1] ? 0 : task[1] - remain;
      remain = max(task[1] - task[0], remain - task[0]);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 2}, {2, 4}, {4, 8}}, 8},
      {{{1, 3}, {2, 4}, {10, 11}, {10, 12}, {8, 9}}, 32},
      {{{1, 7}, {2, 8}, {3, 9}, {4, 10}, {5, 11}, {6, 12}}, 27},
  };

  for (auto &[tasks, expect] : tests) {
    assert(Solution().minimumEffort(tasks) == expect);
  }
}