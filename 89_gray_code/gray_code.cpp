/*
 * @Date: 2022-01-08 01:08:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-08 01:20:30
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> grayCode(int n) {
    vector<int> ret(1 << n);
    for (size_t i = 0; i < ret.size(); i++) {
      ret[i] = (i >> 1) ^ i;
    }
    return ret;
  }
};

int main() {
  assert(Solution().grayCode(2) == vector<int>({0, 1, 3, 2}));
  assert(Solution().grayCode(1) == vector<int>({0, 1}));
}