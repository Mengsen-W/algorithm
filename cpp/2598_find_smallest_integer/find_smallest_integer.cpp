#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findSmallestInteger(vector<int> &nums, int value) {
    vector<int> mp(value);
    for (auto &x : nums) {
      int v = (x % value + value) % value;
      mp[v]++;
    }
    int mex = 0;
    while (mp[mex % value] > 0) {
      mp[mex % value]--;
      mex++;
    }
    return mex;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, -10, 7, 13, 6, 8}, 5, 4},
      {{1, -10, 7, 13, 6, 8}, 7, 2},
  };

  for (auto &[nums, value, ans] : tests) {
    assert(Solution().findSmallestInteger(nums, value) == ans);
  }
}