/*
 * @Date: 2022-12-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-02
 * @FilePath: /algorithm/1769_min_operations/min_operations.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> minOperations(string boxes) {
    int left = boxes[0] - '0', right = 0, operations = 0;
    int n = boxes.size();
    for (int i = 1; i < n; i++) {
      if (boxes[i] == '1') {
        right++;
        operations += i;
      }
    }
    vector<int> res(n);
    res[0] = operations;
    for (int i = 1; i < n; i++) {
      operations += left - right;
      if (boxes[i] == '1') {
        left++;
        right--;
      }
      res[i] = operations;
    }
    return res;
  }
};

int main() {
  {
    string box{"110"};
    vector<int> ans{1, 1, 3};
    assert(Solution().minOperations(box) == ans);
  }

  {
    string box{"001011"};
    vector<int> ans{11, 8, 5, 4, 3, 4};
    assert(Solution().minOperations(box) == ans);
  }
}