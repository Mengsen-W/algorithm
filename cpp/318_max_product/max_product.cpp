/*
 * @Date: 2021-11-16 23:38:50
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-10
 */

#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxProduct(vector<string> words) {
    unordered_map<int, int> map;
    int length = words.size();
    for (int i = 0; i < length; i++) {
      int mask = 0;
      string word = words[i];
      int wordLength = word.size();
      for (int j = 0; j < wordLength; j++) mask |= 1 << (word[j] - 'a');
      if (map.count(mask)) {
        if (wordLength > map[mask]) map[mask] = wordLength;
      } else {
        map[mask] = wordLength;
      }
    }
    int maxProd = 0;
    for (auto [mask1, _] : map) {
      int wordLength1 = map[mask1];
      for (auto [mask2, _] : map) {
        if ((mask1 & mask2) == 0) {
          int wordLength2 = map[mask2];
          maxProd = max(maxProd, wordLength1 * wordLength2);
        }
      }
    }
    return maxProd;
  }
};

int main() {
  vector<tuple<vector<string>, int>> tests{
      {{"abcw", "baz", "foo", "bar", "xtfn", "abcdef"}, 16},
      {{"a", "ab", "abc", "d", "cd", "bcd", "abcd"}, 4},
      {{"a", "aa", "aaa", "aaaa"}, 0},
  };

  for (auto &[word, ans] : tests) {
    assert(Solution().maxProduct(word) == ans);
  }
}