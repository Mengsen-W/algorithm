/*
 * @Date: 2023-05-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-18
 * @FilePath: /algorithm/cpp/1073_add_negabinary/add_negabinary.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> addNegabinary(vector<int>& arr1, vector<int>& arr2) {
    int i = arr1.size() - 1, j = arr2.size() - 1;
    int carry = 0;
    vector<int> ans;
    while (i >= 0 || j >= 0 || carry) {
      int x = carry;
      if (i >= 0) {
        x += arr1[i];
      }
      if (j >= 0) {
        x += arr2[j];
      }

      if (x >= 2) {
        ans.push_back(x - 2);
        carry = -1;
      } else if (x >= 0) {
        ans.push_back(x);
        carry = 0;
      } else {
        ans.push_back(1);
        carry = 1;
      }
      --i;
      --j;
    }
    while (ans.size() > 1 && ans.back() == 0) {
      ans.pop_back();
    }
    reverse(ans.begin(), ans.end());
    return ans;
  }
};

int main() {
  {
    vector<int> arr1{1, 1, 1, 1, 1};
    vector<int> arr2 = {1, 0, 1};
    vector<int> ans{1, 0, 0, 0, 0};
    assert(Solution().addNegabinary(arr1, arr2) == ans);
  }

  {
    vector<int> arr1{0};
    vector<int> arr2 = {0};
    vector<int> ans{0};
    assert(Solution().addNegabinary(arr1, arr2) == ans);
  }

  {
    vector<int> arr1{0};
    vector<int> arr2 = {1};
    vector<int> ans{1};
    assert(Solution().addNegabinary(arr1, arr2) == ans);
  }
}