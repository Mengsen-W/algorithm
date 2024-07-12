#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> numberGame(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    for (int i = 0; i < nums.size(); i += 2) {
      swap(nums[i], nums[i + 1]);
    }
    return nums;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{5, 4, 2, 3}, {3, 2, 5, 4}},
      {{2, 5}, {5, 2}},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().numberGame(nums) == ans);
  }
}