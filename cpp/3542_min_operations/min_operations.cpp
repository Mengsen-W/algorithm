#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int minOperations(vector<int>& nums) {
    vector<int> s;
    int res = 0;
    for (int a : nums) {
      while (!s.empty() && s.back() > a) {
        s.pop_back();
      }
      if (a == 0) continue;
      if (s.empty() || s.back() < a) {
        ++res;
        s.push_back(a);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{0, 2}, 1},
      {{3, 1, 2, 1}, 3},
      {{1, 2, 1, 2, 1, 2}, 4},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().minOperations(nums) == ans);
  }
}
