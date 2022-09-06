/*
 * @Date: 2022-09-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-06
 * @FilePath: /algorithm/828_unique_letter_string/unique_letter_string.cpp
 */

#include <assert.h>

#include <string>
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
  assert(Solution().uniqueLetterString("ABC") == 10);
  assert(Solution().uniqueLetterString("ABA") == 8);
  assert(Solution().uniqueLetterString("LEETCODE") == 92);
}