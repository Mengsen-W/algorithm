/*
 * @Date: 2023-05-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-25
 * @FilePath: /algorithm/cpp/2451_odd_string/odd_string.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> get(string& word) {
    vector<int> diff(word.size() - 1);
    for (int i = 0; i + 1 < word.size(); i++) {
      diff[i] = word[i + 1] - word[i];
    }
    return diff;
  }

  string oddString(vector<string>& words) {
    auto diff0 = get(words[0]);
    auto diff1 = get(words[1]);
    if (diff0 == diff1) {
      for (int i = 2; i < words.size(); i++) {
        if (diff0 != get(words[i])) {
          return words[i];
        }
      }
    }
    return diff0 == get(words[2]) ? words[1] : words[0];
  }
};

int main() {
  {
    vector<string> words{"adc", "wzy", "abc"};
    string ans = "abc";
    assert(Solution().oddString(words) == ans);
  }

  {
    vector<string> words{"aaa", "bob", "ccc", "ddd"};
    string ans = "bob";
    assert(Solution().oddString(words) == ans);
  }
}