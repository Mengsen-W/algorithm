#include <cassert>
#include <vector>
#include <tuple>

using namespace std;

class Solution {
 public:
  vector<bool> prefixesDivBy5(vector<int>& nums) {
    vector<bool> answer;
    int prefix = 0;
    int length = nums.size();
    for (int i = 0; i < length; i++) {
      prefix = ((prefix << 1) + nums[i]) % 5;
      answer.emplace_back(prefix == 0);
    }
    return answer;
  }
};

int main() {
  vector<tuple<vector<int>, vector<bool>>> tests{
      {{0, 1, 1}, {true, false, false}},
      {{1, 1, 1}, {false, false, false}},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().prefixesDivBy5(nums) == ans);
  }
}