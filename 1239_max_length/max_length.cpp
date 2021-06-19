/*
 * @Date: 2021-06-19 09:43:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-19 10:14:34
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

int maxLength(vector<string> &arr) {
  int ans = 0;
  vector<int> masks = {0};
  for (string &s : arr) {
    int mask = 0;
    for (char ch : s) {
      ch -= 'a';
      if ((mask >> ch) & 1) {
        // 若 mask 已有 ch，则说明 s 含有重复字母，无法构成可行解
        mask = 0;
        break;
      }
      // 将 ch 加入 mask 中
      mask |= 1 << ch;
    }
    if (mask == 0) {
      continue;
    }
    int n = masks.size();
    for (int i = 0; i < n; ++i) {
      int m = masks[i];
      if ((m & mask) == 0) {
        // m 和 mask 无公共元素
        masks.push_back(m | mask);
        ans = max(ans, __builtin_popcount(m | mask));
      }
    }
  }
  return ans;
}

int main() {
  {
    vector<string> arr{"un", "iq", "ue"};
    assert(maxLength(arr) == 4);
  }
  {
    vector<string> arr{"cha", "r", "act", "ers"};
    assert(maxLength(arr) == 6);
  }
  {
    vector<string> arr{"abcdefghijklmnopqrstuvwxyz"};
    assert(maxLength(arr) == 26);
  }
}
