/*
 * @Date: 2022-03-01 00:06:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-01 00:27:11
 * @FilePath: /algorithm/6_convert/convert.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution1 {
 public:
  string convert(string s, int numRows) {
    int n = s.length(), r = numRows;
    if (r == 1 || r >= n) {
      return s;
    }
    int t = r * 2 - 2;
    int c = (n + t - 1) / t * (r - 1);
    vector<string> mat(r, string(c, 0));
    for (int i = 0, x = 0, y = 0; i < n; ++i) {
      mat[x][y] = s[i];
      if (i % t < r - 1) {
        ++x;  // 向下移动
      } else {
        --x;
        ++y;  // 向右上移动
      }
    }
    string ans;
    for (auto &row : mat) {
      for (char ch : row) {
        if (ch) {
          ans += ch;
        }
      }
    }
    return ans;
  }
};

class Solution2 {
 public:
  string convert(string s, int numRows) {
    int n = s.length(), r = numRows;
    if (r == 1 || r >= n) {
      return s;
    }
    vector<string> mat(r);
    for (int i = 0, x = 0, t = r * 2 - 2; i < n; ++i) {
      mat[x] += s[i];
      i % t < r - 1 ? ++x : --x;
    }
    string ans;
    for (auto &row : mat) {
      ans += row;
    }
    return ans;
  }
};

class Solution3 {
 public:
  string convert(string s, int numRows) {
    int n = s.length(), r = numRows;
    if (r == 1 || r >= n) {
      return s;
    }
    string ans;
    int t = r * 2 - 2;
    for (int i = 0; i < r; ++i) {           // 枚举矩阵的行
      for (int j = 0; j + i < n; j += t) {  // 枚举每个周期的起始下标
        ans += s[j + i];                    // 当前周期的第一个字符
        if (0 < i && i < r - 1 && j + t - i < n) {
          ans += s[j + t - i];  // 当前周期的第二个字符
        }
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution1().convert("PAYPALISHIRING", 3) == "PAHNAPLSIIGYIR");
  assert(Solution2().convert("PAYPALISHIRING", 3) == "PAHNAPLSIIGYIR");
  assert(Solution3().convert("PAYPALISHIRING", 3) == "PAHNAPLSIIGYIR");

  assert(Solution1().convert("PAYPALISHIRING", 4) == "PINALSIGYAHRPI");
  assert(Solution2().convert("PAYPALISHIRING", 4) == "PINALSIGYAHRPI");
  assert(Solution3().convert("PAYPALISHIRING", 4) == "PINALSIGYAHRPI");

  assert(Solution1().convert("A", 1) == "A");
  assert(Solution2().convert("A", 1) == "A");
  assert(Solution3().convert("A", 1) == "A");
}