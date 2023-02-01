/*
 * @Date: 2023-02-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-01
 * @FilePath: /algorithm/cpp/2325_decode_message/decode_message.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
 public:
  string decodeMessage(string key, string message) {
    char cur = 'a';
    unordered_map<char, char> rules;

    for (char c : key) {
      if (c != ' ' && !rules.count(c)) {
        rules[c] = cur;
        ++cur;
      }
    }

    for (char& c : message) {
      if (c != ' ') {
        c = rules[c];
      }
    }

    return message;
  }
};

int main() {
  {
    string key = "the quick brown fox jumps over the lazy dog";
    string message = "vkbs bs t suepuv";
    string ans = "this is a secret";
    assert(Solution().decodeMessage(key, message) == ans);
  }

  {
    string key = "eljuxhpwnyrdgtqkviszcfmabo";
    string message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb";
    string ans = "the five boxing wizards jump quickly";
    assert(Solution().decodeMessage(key, message) == ans);
  }
}