/*
 * @Date: 2022-10-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-23
 * @FilePath: /algorithm/1768_merge_alternately/merge_alternately.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string mergeAlternately(string word1, string word2) {
    int m = word1.size(), n = word2.size();
    int i = 0, j = 0;

    string ans;
    ans.reserve(m + n);
    while (i < m || j < n) {
      if (i < m) {
        ans.push_back(word1[i]);
        ++i;
      }
      if (j < n) {
        ans.push_back(word2[j]);
        ++j;
      }
    }
    return ans;
  }
};

int main() {
  {
    string word1{"abc"};
    string word2{"pqr"};
    string ans{"apbqcr"};
    assert(Solution().mergeAlternately(word1, word2) == ans);
  }

  {
    string word1{"ab"};
    string word2{"pqrs"};
    string ans{"apbqrs"};
    assert(Solution().mergeAlternately(word1, word2) == ans);
  }

  {
    string word1{"abcd"};
    string word2{"pq"};
    string ans{"apbqcd"};
    assert(Solution().mergeAlternately(word1, word2) == ans);
  }
}