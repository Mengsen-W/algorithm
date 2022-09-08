/*
 * @Date: 2022-09-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-08
 * @FilePath: /algorithm/667_construct_array/construct_array.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> constructArray(int n, int k) {
    vector<int> answer;
    for (int i = 1; i < n - k; ++i) {
      answer.push_back(i);
    }
    for (int i = n - k, j = n; i <= j; ++i, --j) {
      answer.push_back(i);
      if (i != j) {
        answer.push_back(j);
      }
    }
    return answer;
  }
};

int main() {
  {
    int n = 3;
    int k = 1;
    vector<int> ans{1, 2, 3};
    assert(Solution().constructArray(n, k) == ans);
  }

  {
    int n = 3;
    int k = 2;
    vector<int> ans{1, 3, 2};
    assert(Solution().constructArray(n, k) == ans);
  }
}