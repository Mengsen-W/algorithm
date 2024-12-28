#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isPossibleToSplit(vector<int>& nums) {
    unordered_map<int, int> count;
    for (int num : nums) {
      if (++count[num] > 2) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{1, 1, 2, 2, 3, 4}, true},
      {{1, 1, 1, 1}, false},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().isPossibleToSplit(nums) == ans);
  }
}