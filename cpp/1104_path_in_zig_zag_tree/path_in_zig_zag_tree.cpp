/*
 * @Date: 2021-07-29 09:47:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-29 09:57:42
 */

#include <algorithm>
#include <cassert>
#include <cmath>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> pathInZigZagTree(int label) {
    vector<int> res;
    while (label >= 1) {
      res.emplace_back(label);
      label >>= 1;
    }

    // 反转后为正常层序序列
    reverse(res.begin(), res.end());
    // 从倒数第二位隔位修改节点
    for (int i = (int)res.size() - 2; i > 0; i -= 2) {
      res[i] = pow(2, i) + pow(2, i + 1) - 1 - res[i];
    }

    return res;
  }
};

int main() {
  {
    int label = 14;
    vector<int> ans{1, 3, 4, 14};
  }
  {
    int label = 26;
    vector<int> ans{1, 2, 6, 10, 26};
  }
}