/*
 * @Date: 2022-08-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-06
 * @FilePath: /algorithm/1408_string_matching/string_matching.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> stringMatching(vector<string>& words) {
    vector<string> ret;
    for (int i = 0; i < words.size(); i++) {
      for (int j = 0; j < words.size(); j++) {
        if (i != j && words[j].find(words[i]) != string::npos) {
          ret.push_back(words[i]);
          break;
        }
      }
    }
    return ret;
  }
};

int main() {
  {
    vector<string> words{"mass", "as", "hero", "superhero"};
    vector<string> ans{"as", "hero"};
    assert(Solution().stringMatching(words) == ans);
  }

  {
    vector<string> words{"leetcode", "et", "code"};
    vector<string> ans{"et", "code"};
    assert(Solution().stringMatching(words) == ans);
  }

  {
    vector<string> words{"blue", "green", "bu"};
    vector<string> ans{};
    assert(Solution().stringMatching(words) == ans);
  }
}