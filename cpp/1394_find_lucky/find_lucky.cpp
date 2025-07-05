#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  unordered_map<int, int> m;
  int findLucky(vector<int>& arr) {
    for (auto x : arr) {
      ++m[x];
    }
    int ans = -1;
    for (auto [key, value] : m) {
      if (key == value) {
        ans = max(ans, key);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 2, 3, 4}, 2},
      {{1, 2, 2, 3, 3, 3}, 3},
      {{2, 2, 2, 3, 3}, -1},
      {{5}, -1},
      {{7, 7, 7, 7, 7, 7, 7}, 7},
  };

  for (auto &[arr, expect] : tests) {
    assert(Solution().findLucky(arr) == expect);
  }

}