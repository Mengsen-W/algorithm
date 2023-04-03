/*
 * @Date: 2023-04-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-03
 * @FilePath: /algorithm/cpp/1053_prev_perm_opt1/prev_perm_opt1.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> prevPermOpt1(vector<int>& arr) {
    int n = arr.size();
    for (int i = n - 2; i >= 0; i--) {
      if (arr[i] > arr[i + 1]) {
        int j = n - 1;
        while (arr[j] >= arr[i] || arr[j] == arr[j - 1]) {
          j--;
        }
        swap(arr[i], arr[j]);
        break;
      }
    }
    return arr;
  }
};

int main() {
  {
    vector<int> arr{3, 2, 1};
    vector<int> ans{3, 1, 2};
    assert(Solution().prevPermOpt1(arr) == ans);
  }

  {
    vector<int> arr{1, 1, 5};
    vector<int> ans{1, 1, 5};
    assert(Solution().prevPermOpt1(arr) == ans);
  }

  {
    vector<int> arr{1, 9, 4, 6, 7};
    vector<int> ans{1, 7, 4, 6, 9};
    assert(Solution().prevPermOpt1(arr) == ans);
  }
}