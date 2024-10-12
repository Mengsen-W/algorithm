#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int duplicateNumbersXOR(vector<int>& nums) {
    unordered_set<int> cnt;
    int res = 0;
    for (int num : nums) {
      if (cnt.find(num) != cnt.end()) {
        res ^= num;
      } else {
        cnt.emplace(num);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 1, 3}, 1},
      {{1, 2, 3}, 0},
      {{1, 2, 2, 1}, 3},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().duplicateNumbersXOR(nums) == ans);
  }
}
