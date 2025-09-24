#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxFrequencyElements(vector<int>& nums) {
    unordered_map<int, int> count;
    for (int a : nums) {
      count[a]++;
    }
    int maxf = 0;
    for (auto const& pair : count) {
      if (pair.second > maxf) {
        maxf = pair.second;
      }
    }
    int res = 0;
    for (auto const& pair : count) {
      if (pair.second == maxf) {
        res += maxf;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 2, 3, 1, 4}, 4},
      {{1, 2, 3, 4, 5}, 5},
  };

  for (auto& [nums, expect] : tests) {
    assert(Solution().maxFrequencyElements(nums) == expect);
  }
}
