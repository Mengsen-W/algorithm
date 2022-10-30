/*
 * @Date: 2022-10-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-30
 * @FilePath: /algorithm/784_letter_case_permutation/letter_case_permutation.cpp
 */

#include <cassert>
#include <queue>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> letterCasePermutation(string s) {
    vector<string> ans;
    queue<string> qu;
    qu.emplace("");
    while (!qu.empty()) {
      string &curr = qu.front();
      if (curr.size() == s.size()) {
        ans.emplace_back(curr);
        qu.pop();
      } else {
        int pos = curr.size();
        if (isalpha(s[pos])) {
          string next = curr;
          next.push_back(s[pos] ^ 32);
          qu.emplace(next);
        }
        curr.push_back(s[pos]);
      }
    }
    return ans;
  }
};

int main() {
  {
    string s{"a1b2"};
    vector<string> ans{"A1B2", "A1b2", "a1B2", "a1b2"};
    assert(Solution().letterCasePermutation(s) == ans);
  }

  {
    string s{"3z4"};
    vector<string> ans{"3Z4", "3z4"};
    assert(Solution().letterCasePermutation(s) == ans);
  }
}