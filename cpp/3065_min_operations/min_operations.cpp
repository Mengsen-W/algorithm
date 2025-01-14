#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(vector<int>& nums, int k) {
    int res = 0;
    for (int num : nums) {
      if (num < k) {
        res++;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{2, 11, 10, 1, 3}, 10, 3},
      {{1, 1, 2, 4, 9}, 1, 0},
      {{1, 1, 2, 4, 9}, 9, 4},
  };
  
  for (auto &[nums, k, ans] : tests ) {
    assert(Solution().minOperations(nums, k) == ans);
  }
}