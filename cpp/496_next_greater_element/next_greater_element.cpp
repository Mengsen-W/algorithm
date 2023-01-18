/*
 * @Date: 2021-10-26 01:01:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-26 01:06:42
 */

#include <cassert>
#include <stack>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> nextGreaterElement(vector<int>& nums1, vector<int>& nums2) {
    unordered_map<int, int> hashmap;
    stack<int> st;
    for (int i = nums2.size() - 1; i >= 0; --i) {
      int num = nums2[i];
      while (!st.empty() && num >= st.top()) st.pop();
      hashmap[num] = st.empty() ? -1 : st.top();
      st.push(num);
    }
    vector<int> res(nums1.size());
    for (int i = 0; i < (int)nums1.size(); ++i) res[i] = hashmap[nums1[i]];
    return res;
  }
};

int main() {
  {
    vector<int> nums1{4, 1, 2};
    vector<int> nums2{1, 3, 4, 2};
    vector<int> ans{-1, 3, -1};
    assert(Solution().nextGreaterElement(nums1, nums2) == ans);
  }
  {
    vector<int> nums1{2, 4};
    vector<int> nums2{1, 2, 3, 4};
    vector<int> ans{3, -1};
    assert(Solution().nextGreaterElement(nums1, nums2) == ans);
  }
}