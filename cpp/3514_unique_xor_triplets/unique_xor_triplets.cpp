#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int uniqueXorTriplets(vector<int>& nums) {
    int m = 0;
    for (int v : nums) {
      m = max(m, v);
    }
    int u = 1;
    while (u <= m) {
      u <<= 1;
    }
    vector<int> one(u), two(u), three(u);
    for (int v : nums) {
      one[v] = 1;
      for (int x = 0; x < u; x++) {
        if (one[x]) {
          two[x ^ v] = 1;
        }
      }
    }
    for (int v : nums) {
      for (int x = 0; x < u; x++) {
        if (two[x]) {
          three[x ^ v] = 1;
        }
      }
    }
    int ans = 0;
    for (int x = 0; x < u; x++) {
      if (three[x]) {
        ans++;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 3}, 2},
      {{6, 7, 8, 9}, 4},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().uniqueXorTriplets(nums) == expected);
  }
  return 0;
}