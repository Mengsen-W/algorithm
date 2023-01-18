/*
 * @Date: 2022-01-27 02:37:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-27 02:51:23
 */

#include <cassert>
#include <string>
#include <string_view>

using namespace std;

class Solution {
 public:
  int countValidWords(string sentence) {
    int n = sentence.length();
    int l = 0, r = 0;
    int ret = 0;
    string_view slice(sentence);
    while (true) {
      while (l < n && sentence[l] == ' ') {
        l++;
      }
      if (l >= n) {
        break;
      }
      r = l + 1;
      while (r < n && sentence[r] != ' ') {
        r++;
      }
      if (isValid(slice.substr(
              l, r - l))) {  // 判断根据空格分解出来的 token 是否有效
        ret++;
      }
      l = r + 1;
    }
    return ret;
  }

  bool isValid(const string_view &word) {
    int n = word.length();
    bool has_hyphens = false;
    for (int i = 0; i < n; i++) {
      if (word[i] >= '0' && word[i] <= '9') {
        return false;
      } else if (word[i] == '-') {
        if (has_hyphens == true || i == 0 || i == n - 1 ||
            !islower(word[i - 1]) || !islower(word[i + 1])) {
          return false;
        }
        has_hyphens = true;
      } else if (word[i] == '!' || word[i] == '.' || word[i] == ',') {
        if (i != n - 1) {
          return false;
        }
      }
    }
    return true;
  }
};

int main() {
  assert(Solution().countValidWords("cat and  dog") == 3);
  assert(Solution().countValidWords("!this  1-s b8d!") == 0);
  assert(Solution().countValidWords(
             "alice and  bob are playing stone-game10") == 5);
  assert(Solution().countValidWords(
             "he bought 2 pencils, 3 erasers, and 1  pencil-sharpener.") == 6);
}