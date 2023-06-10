/*
 * @Date: 2023-06-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-10
 * @FilePath: /algorithm/cpp/1170_num_smaller_by_frequency/num_smaller_by_frequency.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int f(string &s) {
    int cnt = 0;
    char ch = 'z';
    for (auto c : s) {
      if (c < ch) {
        ch = c;
        cnt = 1;
      } else if (c == ch) {
        cnt++;
      }
    }
    return cnt;
  }

  vector<int> numSmallerByFrequency(vector<string> &queries, vector<string> &words) {
    vector<int> count(12);
    for (auto &s : words) {
      count[f(s)]++;
    }
    for (int i = 9; i >= 1; i--) {
      count[i] += count[i + 1];
    }
    vector<int> res;
    for (auto &s : queries) {
      res.push_back(count[f(s) + 1]);
    }
    return res;
  }
};

int main() {
  {
    vector<string> queries{"cbd"};
    vector<string> words{"zaaaz"};
    vector<int> ans{1};
    assert(Solution().numSmallerByFrequency(queries, words) == ans);
  }

  {
    vector<string> queries{"bbb", "cc"};
    vector<string> words{"a", "aa", "aaa", "aaaa"};
    vector<int> ans{1, 2};
    assert(Solution().numSmallerByFrequency(queries, words) == ans);
  }
}