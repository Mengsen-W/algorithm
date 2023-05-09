/*
 * @Date: 2023-05-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-09
 * @FilePath: /algorithm/cpp/2437_count_time/count_time.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int countTime(string time) {
    int countHour = 0;
    int countMinute = 0;
    for (int i = 0; i < 24; i++) {
      int hiHour = i / 10;
      int loHour = i % 10;
      if ((time[0] == '?' || time[0] == hiHour + '0') && (time[1] == '?' || time[1] == loHour + '0')) {
        countHour++;
      }
    }
    for (int i = 0; i < 60; i++) {
      int hiMinute = i / 10;
      int loMinute = i % 10;
      if ((time[3] == '?' || time[3] == hiMinute + '0') && (time[4] == '?' || time[4] == loMinute + '0')) {
        countMinute++;
      }
    }
    return countHour * countMinute;
  }
};

int main() {
  {
    string time = "?5:00";
    int ans = 2;
    assert(Solution().countTime(time) == ans);
  }

  {
    string time = "0?:0?";
    int ans = 100;
    assert(Solution().countTime(time) == ans);
  }

  {
    string time = "??:??";
    int ans = 1440;
    assert(Solution().countTime(time) == ans);
  }
}