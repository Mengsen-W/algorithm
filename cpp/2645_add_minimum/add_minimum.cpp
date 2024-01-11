/*
 * @Date: 2024-01-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-11
 * @FilePath: /algorithm/cpp/2645_add_minimum/add_minimum.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int addMinimum(string word) {
    int n = word.size(), cnt = 1;
    for (int i = 1; i < n; i++) {
      if (word[i] <= word[i - 1]) {
        cnt++;
      }
    }
    return cnt * 3 - n;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"b", 2},
      {"aaa", 6},
      {"abc", 0},
  };

  for (auto& [word, ans] : tests) {
    assert(Solution().addMinimum(word) == ans);
  }
}
