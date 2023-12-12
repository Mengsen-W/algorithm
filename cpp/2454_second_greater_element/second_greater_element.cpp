/*
 * @Date: 2023-12-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-12
 * @FilePath: /algorithm/cpp/2454_second_greater_element/second_greater_element.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> secondGreaterElement(vector<int> &nums) {
    int n = nums.size();
    vector<int> res(n, -1);
    vector<int> st1;
    vector<int> st2;
    for (int i = 0; i < n; ++i) {
      int v = nums[i];
      while (!st2.empty() && nums[st2.back()] < v) {
        res[st2.back()] = v;
        st2.pop_back();
      }
      int pos = st1.size() - 1;
      while (pos >= 0 && nums[st1[pos]] < v) {
        --pos;
      }
      st2.insert(st2.end(), st1.begin() + (pos + 1), st1.end());
      st1.resize(pos + 1);
      st1.push_back(i);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{2, 4, 0, 9, 6}, {9, 6, 6, -1, -1}},
      {{3, 3}, {-1, -1}},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().secondGreaterElement(nums) == ans);
  }
}