/*
 * @Date: 2024-01-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-12
 * @FilePath: /algorithm/cpp/2085_count_words/count_words.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int countWords(vector<string>& words1, vector<string>& words2) {
    // 统计字符串出现频率
    unordered_map<string, int> freq1, freq2;
    for (const string& w : words1) {
      ++freq1[w];
    }
    for (const string& w : words2) {
      ++freq2[w];
    }

    // 遍历 words1 出现的字符并判断是否满足要求
    int res = 0;
    for (const auto& [w, cnt1] : freq1) {
      if (cnt1 == 1 && freq2[w] == 1) {
        ++res;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<string>, vector<string>, int>> tests{
      {{"leetcode", "is", "amazing", "as", "is"}, {"amazing", "leetcode", "is"}, 2},
      {{"b", "bb", "bbb"}, {"a", "aa", "aaa"}, 0},
      {{"a", "ab"}, {"a", "a", "a", "ab"}, 1},
  };

  for (auto& [words1, words2, ans] : tests) {
    assert(Solution().countWords(words1, words2) == ans);
  }
}