#include <cassert>
#include <map>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int beautifulSubsets(vector<int>& nums, int k) {
    unordered_map<int, map<int, int>> groups;
    for (int a : nums) {
      ++groups[a % k][a];
    }
    int ans = 1;
    for (auto& [_, g] : groups) {
      int m = g.size();
      vector<vector<int>> f(m, vector<int>(2, 0));
      f[0][0] = 1, f[0][1] = (1 << g.begin()->second) - 1;
      int i = 1;
      for (auto it = next(g.begin()); it != g.end(); it++, i++) {
        f[i][0] = f[i - 1][0] + f[i - 1][1];
        if (it->first - prev(it)->first == k) {
          f[i][1] = f[i - 1][0] * ((1 << it->second) - 1);
        } else {
          f[i][1] = (f[i - 1][0] + f[i - 1][1]) * ((1 << it->second) - 1);
        }
      }
      ans *= f[m - 1][0] + f[m - 1][1];
    }
    return ans - 1;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{2, 4, 6}, 2, 4},
      {{1}, 1, 1},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().beautifulSubsets(nums, k) == ans);
  }
}
