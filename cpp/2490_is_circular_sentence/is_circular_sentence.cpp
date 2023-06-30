/*
 * @Date: 2023-06-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-30
 * @FilePath: /algorithm/cpp/2490_is_circular_sentence/is_circular_sentence.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isCircularSentence(string sentence) {
    if (sentence.back() != sentence.front()) {
      return false;
    }
    for (int i = 0; i < sentence.size(); i++) {
      if (sentence[i] == ' ' && sentence[i + 1] != sentence[i - 1]) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  vector<tuple<string, bool>> testMap{
      {"leetcode exercises sound delightful", true},
      {"eetcode", true},
      {"Leetcode is cool", false},
  };

  for (auto &[sentence, expected] : testMap) {
    assert(Solution().isCircularSentence(sentence) == expected);
  }
}
