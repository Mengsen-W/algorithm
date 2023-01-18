/*
 * @Date: 2021-10-27 01:47:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-27 02:42:15
 */

#include <cassert>
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  bool checkValid(const string& str, int lmask, vector<int>& left, int rmask,
                  vector<int>& right) {
    int pos1 = 0;
    int pos2 = 0;
    int cnt = 0;

    int str_size = str.size();
    for (int i = 0; i < str_size; i++) {
      int left_size = left.size();
      int right_size = right.size();
      if (pos1 < left_size && i == left[pos1]) {
        if (!(lmask & (1 << pos1))) cnt++;
        pos1++;
      } else if (pos2 < right_size && i == right[pos2]) {
        if (!(rmask & (1 << pos2))) {
          cnt--;
          if (cnt < 0) return false;
        }
        pos2++;
      }
    }

    return cnt == 0;
  }

  string recoverStr(const string& str, int lmask, vector<int>& left, int rmask,
                    vector<int>& right) {
    string ans;
    int pos1 = 0;
    int pos2 = 0;

    int str_size = str.size();
    for (int i = 0; i < str_size; i++) {
      int left_size = left.size();
      int right_size = right.size();
      if (pos1 < left_size && i == left[pos1]) {
        if (!(lmask & (1 << pos1))) ans.push_back(str[i]);

        pos1++;
      } else if (pos2 < right_size && i == right[pos2]) {
        if (!(rmask & (1 << pos2))) ans.push_back(str[i]);

        pos2++;
      } else {
        ans.push_back(str[i]);
      }
    }

    return ans;
  }

  vector<string> removeInvalidParentheses(string s) {
    int lremove = 0;
    int rremove = 0;
    vector<int> left;
    vector<int> right;
    vector<string> ans;
    unordered_set<string> cnt;

    int s_size = s.size();
    for (int i = 0; i < s_size; i++) {
      if (s[i] == '(') {
        left.push_back(i);
        lremove++;
      } else if (s[i] == ')') {
        right.push_back(i);
        if (lremove == 0)
          rremove++;
        else
          lremove--;
      }
    }

    int m = left.size();
    int n = right.size();
    vector<int> maskArr1;
    vector<int> maskArr2;
    for (int i = 0; i < (1 << m); i++) {
      if (__builtin_popcount(i) != lremove) continue;

      maskArr1.push_back(i);
    }
    for (int j = 0; j < (1 << n); j++) {
      if (__builtin_popcount(j) != rremove) continue;

      maskArr2.push_back(j);
    }
    for (auto mask1 : maskArr1)
      for (auto mask2 : maskArr2)
        if (checkValid(s, mask1, left, mask2, right))
          cnt.insert(recoverStr(s, mask1, left, mask2, right));

    for (auto v : cnt) ans.emplace_back(v);

    return ans;
  }
};

int main() {
  {
    string s = "()())()";
    vector<string> ans{"()()()", "(())()"};
    assert(Solution().removeInvalidParentheses(s) == ans);
  }
  {
    string s = "(a)())()";
    vector<string> ans{"(a)()()", "(a())()"};
    assert(Solution().removeInvalidParentheses(s) == ans);
  }
  {
    string s = ")(";
    vector<string> ans{""};
    assert(Solution().removeInvalidParentheses(s) == ans);
  }
}