/*
 * @Date: 2022-11-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-01
 * @FilePath: /algorithm/1662_array_strings_are_equal/array_strings_are_equal.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  bool arrayStringsAreEqual(vector<string>& word1, vector<string>& word2) {
    int p1 = 0, p2 = 0, i = 0, j = 0;
    while (p1 < word1.size() && p2 < word2.size()) {
      if (word1[p1][i] != word2[p2][j]) {
        return false;
      }
      i++;
      if (i == word1[p1].size()) {
        p1++;
        i = 0;
      }
      j++;
      if (j == word2[p2].size()) {
        p2++;
        j = 0;
      }
    }
    return p1 == word1.size() && p2 == word2.size();
  }
};

int main() {
  {
    vector<string> word1{"ab", "c"};
    vector<string> word2{"a", "bc"};
    assert(Solution().arrayStringsAreEqual(word1, word2));
  }

  {
    vector<string> word1{"a", "cb"};
    vector<string> word2{"ab", "c"};
    assert(!Solution().arrayStringsAreEqual(word1, word2));
  }

  {
    vector<string> word1{"abc", "d", "defg"};
    vector<string> word2{"abcddefg"};
    assert(Solution().arrayStringsAreEqual(word1, word2));
  }
}