#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int findFinalValue(vector<int>& nums, int original) {
    unordered_set<int> s(nums.begin(), nums.end());
    while (s.count(original)) {
      original *= 2;
    }
    return original;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{5, 3, 6, 1, 12}, 3, 24},
      {{2, 7, 9}, 4, 4},
  };

  for (auto& [nums, original, ans] : tests) {
    assert(Solution().findFinalValue(nums, original) == ans);
  }
}