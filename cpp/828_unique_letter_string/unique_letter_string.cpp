/*
 * @Date: 2022-09-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-26
 * @FilePath: /algorithm/cpp/828_unique_letter_string/unique_letter_string.cpp
 */

#include <assert.h>

#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int uniqueLetterString(string s) {
    unordered_map<char, vector<int>> index;
    for (int i = 0; i < s.size(); i++) {
      index[s[i]].emplace_back(i);
    }
    int res = 0;
    for (auto &&[_, arr] : index) {
      arr.insert(arr.begin(), -1);
      arr.emplace_back(s.size());
      for (int i = 1; i < arr.size() - 1; i++) {
        res += (arr[i] - arr[i - 1]) * (arr[i + 1] - arr[i]);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"ABC", 10},
      {"ABA", 8},
      {"LEETCODE", 92},
  };
  for (auto &&[s, ans] : tests) {
    assert(Solution().uniqueLetterString(s) == ans);
  }
}