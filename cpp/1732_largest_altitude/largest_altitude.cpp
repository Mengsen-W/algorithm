#include <cassert>
#include <vector>
#include <tuple>

using namespace std;

class Solution {
 public:
  int largestAltitude(vector<int>& gain) {
    int ans = 0, sum = 0;
    for (int x : gain) {
      sum += x;
      ans = max(ans, sum);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{-5, 1, 5, 0, -7}, 1},
      {{-4, -3, -2, -1, 4, 3, 2}, 0},
  };
  
  for (auto [gain, expected] : tests) {
    assert(Solution().largestAltitude(gain) == expected);
  }
}