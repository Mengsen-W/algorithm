/*
 * @Date: 2021-12-21 01:20:46
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-31
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int dayOfYear(string date) {
    int year = stoi(date.substr(0, 4));
    int month = stoi(date.substr(5, 2));
    int day = stoi(date.substr(8, 2));

    int amount[] = {31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
    if (year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)) {
      ++amount[1];
    }

    int ans = 0;
    for (int i = 0; i < month - 1; ++i) {
      ans += amount[i];
    }
    return ans + day;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"2019-01-09", 9},
      {"2019-02-10", 41},
      {"2003-03-01", 60},
      {"2004-03-01", 61},
  };

  for (auto& [date, ans] : tests) {
    assert(Solution().dayOfYear(date) == ans);
  }
}