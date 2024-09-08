#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> sortedSquares(vector<int>& nums) {
    vector<int> ans;
    for (int num : nums) {
      ans.push_back(num * num);
    }
    sort(ans.begin(), ans.end());
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{-4, -1, 0, 3, 10}, {0, 1, 9, 16, 100}},
      {{-7, -3, 2, 3, 11}, {4, 9, 9, 49, 121}},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().sortedSquares(nums) == ans);
  }
}