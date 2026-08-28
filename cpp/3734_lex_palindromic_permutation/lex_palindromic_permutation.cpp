#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  string lexPalindromicPermutation(string s, string target) {
    int n = s.length();
    // 特殊情况：长度为1
    if (n == 1) {
      return s > target ? s : "";
    }

    // 统计每个字符的出现次数
    vector<int> cnt(26, 0);
    for (char c : s) {
      cnt[c - 'a']++;
    }

    // 检查是否能构成回文串，并记录奇数个的字符
    string oddChar = "";
    for (int i = 0; i < 26; i++) {
      if (cnt[i] % 2 == 1) {
        // 超过一个字符出现奇数次，无法构成回文
        if (oddChar != "") {
          return "";
        }
        oddChar = string(1, 'a' + i);
      }
      cnt[i] /= 2;  // 只需要一半的字符来构造左半部分
    }

    string prefix = "";

    auto check = [&](char c) -> bool {
      string left = prefix;
      left.push_back(c);
      for (int i = 25; i >= 0; i--) {
        left.append(cnt[i], 'a' + i);
      }

      string palindrome = left + oddChar;
      string reversed_left = left;
      reverse(reversed_left.begin(), reversed_left.end());
      palindrome += reversed_left;

      return palindrome > target;
    };

    // 贪心构造左半部分的每一位
    for (int i = 0; i < n / 2; i++) {
      bool found = false;
      // 尝试放置字典序最小的字符
      for (int j = 0; j < 26; j++) {
        if (cnt[j] == 0) {
          continue;
        }

        cnt[j]--;
        if (check('a' + j)) {
          // 如果构造的回文串大于target，则选择该字符
          prefix.push_back('a' + j);
          found = true;
          break;
        } else {
          cnt[j]++;  // 不满足条件，恢复计数
        }
      }
      if (!found) {
        return "";  // 无法构造出大于target的回文串
      }

      if (prefix[i] > target[i]) {  // prefix已经大于target
        string left = prefix;
        for (int j = 0; j < 26; j++) {
          left.append(cnt[j], 'a' + j);
        }
        string palindrome = left + oddChar;
        string reversed_left = left;
        reverse(reversed_left.begin(), reversed_left.end());
        palindrome += reversed_left;
        return palindrome;
      }
    }

    // 构造最终的回文串
    string ans = prefix + oddChar;
    string reversed_prefix = prefix;
    reverse(reversed_prefix.begin(), reversed_prefix.end());
    ans += reversed_prefix;
    return ans;
  }
};

int main() {
  vector<tuple<string, string, string>> tests{
      {"baba", "abba", "baab"},
      {"baba", "bbaa", ""},
      {"abc", "abb", ""},
      {"aac", "abb", "aca"},
  };

  for (const auto& [s, target, ans] : tests) {
    assert(Solution().lexPalindromicPermutation(s, target) == ans);
  }
}