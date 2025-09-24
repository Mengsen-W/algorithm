#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(vector<int>& nums, int k) {
    unordered_set<int> st;
    for (int x : nums) {
      if (x < k) {
        return -1;
      } else if (x > k) {
        st.insert(x);
      }
    }
    return st.size();
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{5, 2, 5, 4, 5}, 2, 2},
      {{2, 1, 2}, 2, -1},
      {{9, 7, 5, 3}, 1, 4},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().minOperations(nums, k) == ans);
  }
}