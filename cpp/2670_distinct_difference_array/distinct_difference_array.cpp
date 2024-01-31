/*
 * @Date: 2024-01-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-31
 * @FilePath: /algorithm/cpp/2670_distinct_difference_array/distinct_difference_array.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> distinctDifferenceArray(vector<int>& nums) {
    int n = nums.size();
    unordered_set<int> st;
    vector<int> sufCnt(n + 1, 0);
    for (int i = n - 1; i > 0; i--) {
      st.insert(nums[i]);
      sufCnt[i] = st.size();
    }
    vector<int> res;
    st.clear();
    for (int i = 0; i < n; i++) {
      st.insert(nums[i]);
      res.push_back(int(st.size()) - sufCnt[i + 1]);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{1, 2, 3, 4, 5}, {-3, -1, 1, 3, 5}},
      {{3, 2, 3, 4, 2}, {-2, -1, 0, 2, 3}},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().distinctDifferenceArray(nums) == ans);
  }
}
