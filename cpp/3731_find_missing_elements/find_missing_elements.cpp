#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> findMissingElements(vector<int>& nums) {
    std::ranges::sort(nums);
    vector<int> missing;
    for (int i = 0; i < nums.size() - 1; ++i) {
      for (int j = nums[i] + 1; j < nums[i + 1]; ++j) {
        missing.push_back(j);
      }
    }
    return missing;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{1, 4, 2, 5}, {3}},
      {{7, 8, 6, 9}, {}},
      {{5, 1}, {2, 3, 4}},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().findMissingElements(nums) == ans);
  }
  return 0;
}