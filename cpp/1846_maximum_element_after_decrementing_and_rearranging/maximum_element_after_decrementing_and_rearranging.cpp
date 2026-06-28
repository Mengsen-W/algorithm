#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int maximumElementAfterDecrementingAndRearranging(vector<int> &arr) {
    int n = arr.size();
    vector<int> cnt(n + 1);
    for (int v : arr) ++cnt[min(v, n)];

    int miss = 0;
    for (int i = 1; i <= n; ++i)
      if (cnt[i] == 0)
        ++miss;
      else
        miss -= min(cnt[i] - 1, miss);  // miss 不会小于 0，故至多减去 miss 个元素

    return n - miss;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 2, 1, 2, 1}, 2},
      {{100, 1, 1000}, 3},
      {{1, 2, 3, 4, 5}, 5},
  };

  for (auto [arr, expected] : tests) {
    assert(Solution().maximumElementAfterDecrementingAndRearranging(arr) == expected);
  }
}