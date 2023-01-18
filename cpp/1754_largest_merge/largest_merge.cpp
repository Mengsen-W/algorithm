/*
 * @Date: 2022-12-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-24
 * @FilePath: /algorithm/1754_largest_merge/largest_merge.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string largestMerge(string word1, string word2) {
    string merge;
    int i = 0, j = 0;
    while (i < word1.size() || j < word2.size()) {
      if (i < word1.size() && word1.substr(i) > word2.substr(j)) {
        merge.push_back(word1[i++]);
      } else {
        merge.push_back(word2[j++]);
      }
    }
    return merge;
  }
};

int main() {
  {
    string word1 = "cabaa", word2 = "bcaaa";
    string ans = "cbcabaaaaa";
    assert(Solution().largestMerge(word1, word2) == ans);
  }

  {
    string word1 = "abcabc", word2 = "abdcaba";
    string ans = "abdcabcabcaba";
    assert(Solution().largestMerge(word1, word2) == ans);
  }
}