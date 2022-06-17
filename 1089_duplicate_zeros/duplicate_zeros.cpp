/*
 * @Date: 2022-06-17 09:39:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-17 09:45:23
 * @FilePath: /algorithm/1089_duplicate_zeros/duplicate_zeros.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  void duplicateZeros(vector<int>& arr) {
    int n = arr.size();
    int top = 0;
    int i = -1;
    while (top < n) {
      i++;
      if (arr[i] != 0) {
        top++;
      } else {
        top += 2;
      }
    }
    int j = n - 1;
    if (top == n + 1) {
      arr[j] = 0;
      j--;
      i--;
    }
    while (j >= 0) {
      arr[j] = arr[i];
      j--;
      if (!arr[i]) {
        arr[j] = arr[i];
        j--;
      }
      i--;
    }
  }
};

int main() {
  {
    vector<int> arr = {1, 0, 2, 3, 0, 4, 5, 0};
    Solution().duplicateZeros(arr);
    assert((arr == vector<int>{1, 0, 0, 2, 3, 0, 0, 4}));
  }
  {
    vector<int> arr = {1, 2, 3};
    Solution().duplicateZeros(arr);
    assert((arr == vector<int>{1, 2, 3}));
  }
}