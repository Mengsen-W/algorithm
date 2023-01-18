/*
 * @Date: 2022-05-17 09:33:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-17 09:37:53
 * @FilePath: /algorithm/953_is_alien_sorted/is_alien_sorted.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isAlienSorted(vector<string> words, string order) {
    vector<int> index(26);
    for (int i = 0; i < order.size(); i++) {
      index[order[i] - 'a'] = i;
    }
    for (int i = 1; i < words.size(); i++) {
      bool valid = false;
      for (int j = 0; j < words[i - 1].size() && j < words[i].size(); j++) {
        int prev = index[words[i - 1][j] - 'a'];
        int curr = index[words[i][j] - 'a'];
        if (prev < curr) {
          valid = true;
          break;
        } else if (prev > curr) {
          return false;
        }
      }
      if (!valid) {
        /* 比较两个字符串的长度 */
        if (words[i - 1].size() > words[i].size()) {
          return false;
        }
      }
    }
    return true;
  }
};

int main() {
  assert(Solution().isAlienSorted(vector<string>{"hello", "leetcode"}, "hlabcdefgijkmnopqrstuvwxyz"));
  assert(!Solution().isAlienSorted(vector<string>{"word", "world", "row"}, "worldabcefghijkmnpqstuvxyz"));
  assert(!Solution().isAlienSorted(vector<string>{"apple", "app"}, "abcdefghijklmnopqrstuvwxyz"));
}