#include <cassert>
#include <stack>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> nextGreaterElements(vector<int>& nums) {
    int n = nums.size();
    vector<int> ret(n, -1);
    stack<int> stk;
    for (int i = 0; i < n * 2 - 1; i++) {
      while (!stk.empty() && nums[stk.top()] < nums[i % n]) {
        ret[stk.top()] = nums[i % n];
        stk.pop();
      }
      stk.push(i % n);
    }
    return ret;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{1, 2, 1}, {2, -1, 2}},
      {{1, 2, 3, 4, 3}, {2, 3, 4, -1, 4}},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().nextGreaterElements(nums) == ans);
  }
  return 0;
}