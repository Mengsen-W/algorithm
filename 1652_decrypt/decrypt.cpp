/*
 * @Date: 2022-09-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-24
 * @FilePath: /algorithm/1652_decrypt/decrypt.cpp
 */

#include <assert.h>

#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> decrypt(vector<int>& code, int k) {
    int n = code.size();
    vector<int> res(n);
    if (k == 0) {
      return res;
    }
    code.resize(n * 2);
    copy(code.begin(), code.begin() + n, code.begin() + n);
    int l = k > 0 ? 1 : n + k;
    int r = k > 0 ? k : n - 1;
    int w = 0;
    for (int i = l; i <= r; i++) {
      w += code[i];
    }
    for (int i = 0; i < n; i++) {
      res[i] = w;
      w -= code[l];
      w += code[r + 1];
      l++, r++;
    }
    return res;
  }
};

int main() {
  {
    vector<int> code{5, 7, 1, 4};
    int k = 3;
    vector<int> ans{12, 10, 16, 13};
    assert(Solution().decrypt(code, k) == ans);
  }

  {
    vector<int> code{1, 2, 3, 4};
    int k = 0;
    vector<int> ans{0, 0, 0, 0};
    assert(Solution().decrypt(code, k) == ans);
  }

  {
    vector<int> code{2, 4, 9, 3};
    int k = -2;
    vector<int> ans{12, 5, 6, 13};
    assert(Solution().decrypt(code, k) == ans);
  }
}