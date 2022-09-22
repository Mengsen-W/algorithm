/*
 * @Date: 2022-09-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-22
 * @FilePath: /algorithm/1604_can_form_array/can_form_array.cpp
 */

#include <assert.h>

#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  bool canFormArray(vector<int> &arr, vector<vector<int>> &pieces) {
    unordered_map<int, int> index;
    for (int i = 0; i < pieces.size(); i++) {
      index[pieces[i][0]] = i;
    }
    for (int i = 0; i < arr.size();) {
      auto it = index.find(arr[i]);
      if (it == index.end()) {
        return false;
      }
      for (int x : pieces[it->second]) {
        if (arr[i++] != x) {
          return false;
        }
      }
    }
    return true;
  }
};

int main() {
  {
    vector<int> arr{15, 88};
    vector<vector<int>> pieces{{88}, {15}};
    assert(Solution().canFormArray(arr, pieces));
  }

  {
    vector<int> arr{49, 18, 16};
    vector<vector<int>> pieces{{16, 18, 49}};
    assert(!Solution().canFormArray(arr, pieces));
  }

  {
    vector<int> arr{91, 4, 64, 78};
    vector<vector<int>> pieces{{78}, {4, 64}, {91}};
    assert(Solution().canFormArray(arr, pieces));
  }
}