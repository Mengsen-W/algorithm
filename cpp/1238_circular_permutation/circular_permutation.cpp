/*
 * @Date: 2023-02-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-23
 * @FilePath: /algorithm/cpp/1238_circular_permutation/circular_permutation.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> circularPermutation(int n, int start) {
    vector<int> ret(1 << n);
    for (size_t i = 0; i < ret.size(); i++) {
      ret[i] = (i >> 1) ^ i ^ start;
    }
    return ret;
  }
};

int main() {
  {
    int n = 2;
    int start = 3;
    vector<int> ans{3, 2, 0, 1};
    assert(Solution().circularPermutation(n, start) == ans);
  }

  {
    int n = 3;
    int start = 2;
    vector<int> ans{2, 3, 1, 0, 4, 5, 7, 6};
    assert(Solution().circularPermutation(n, start) == ans);
  }
}