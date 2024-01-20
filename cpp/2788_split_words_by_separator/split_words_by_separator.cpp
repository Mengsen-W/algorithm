/*
 * @Date: 2024-01-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-20
 * @FilePath: /algorithm/cpp/2788_split_words_by_separator/split_words_by_separator.cpp
 */

#include <cassert>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> splitWordsBySeparator(vector<string>& words, char separator) {
    vector<string> res;
    for (string& word : words) {
      stringstream ss(word);
      string sub;
      while (getline(ss, sub, separator)) {
        if (!sub.empty()) {
          res.push_back(sub);
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<string>, char, vector<string>>> tests{
      {{"one.two.three", "four.five", "six"}, '.', {"one", "two", "three", "four", "five", "six"}},
      {{"$easy$", "$problem$"}, '$', {"easy", "problem"}},
      {{"|||"}, '|', {}},
  };

  for (auto &[words, separator, ans] : tests) {
    assert(Solution().splitWordsBySeparator(words, separator) == ans);
  }
}