/*
 * @Date: 2023-10-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-23
 * @FilePath: /algorithm/cpp/2678_count_seniors/count_seniors.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countSeniors(vector<string>& details) {
    int count = 0;
    for (string& info : details) {
      if (stoi(info.substr(11, 2)) > 60) {
        count++;
      }
    }
    return count;
  }
};

int main() {
  vector<tuple<vector<string>, int>> tests{
      {{"7868190130M7522", "5303914400F9211", "9273338290F4010"}, 2},
      {{"1313579440F2036", "2921522980M5644"}, 0},
  };

  for (auto& [details, ans] : tests) {
    assert(Solution().countSeniors(details) == ans);
  }
}