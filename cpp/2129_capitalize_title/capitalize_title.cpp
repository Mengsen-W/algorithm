/*
 * @Date: 2024-03-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-11
 * @FilePath: /algorithm/cpp/2129_capitalize_title/capitalize_title.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string capitalizeTitle(string title) {
    int n = title.size();
    int l = 0, r = 0;  // 单词左右边界（左闭右开）
    while (r < n) {
      while (r < n && title[r] != ' ') {
        ++r;
      }
      // 对于每个单词按要求处理
      if (r - l > 2) {
        title[l++] = toupper(title[l]);
      }
      while (l < r) {
        title[l++] = tolower(title[l]);
      }
      l = ++r;
    }
    return title;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"capiTalIze tHe titLe", "Capitalize The Title"},
      {"First leTTeR of EACH Word", "First Letter of Each Word"},
      {"i lOve leetcode", "i Love Leetcode"},
  };

  for (auto &[title, ans] : tests) {
    assert(Solution().capitalizeTitle(title) == ans);
  }
}