/*
 * @Date: 2022-09-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-05
 * @FilePath: /algorithm/cpp/1652_decrypt/decrypt.cpp
 */

#include <assert.h>

#include <tuple>
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
  vector<tuple<vector<int>, int, vector<int>>> tests{
      {{5, 7, 1, 4}, 3, {12, 10, 16, 13}},
      {{1, 2, 3, 4}, 0, {0, 0, 0, 0}},
      {{2, 4, 9, 3}, -2, {12, 5, 6, 13}},
  };

  for (auto& [code, k, ans] : tests) {
    assert(Solution().decrypt(code, k) == ans);
  }
}