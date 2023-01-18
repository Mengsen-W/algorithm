/*
 * @Date: 2021-07-24 13:31:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-24 13:38:44
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
public:
  string maximumTime(string time) {
    if (time[0] == '?') {
      time[0] = ('4' <= time[1] && time[1] <= '9') ? '1' : '2';
    }
    if (time[1] == '?') {
      time[1] = (time[0] == '2') ? '3' : '9';
    }
    if (time[3] == '?') {
      time[3] = '5';
    }
    if (time[4] == '?') {
      time[4] = '9';
    }
    return time;
  }
};

int main() {
  {
    string time = "2?:?0";
    assert(Solution{}.maximumTime(time) == "23:50");
  }

  {
    string time = "0?:3?";
    assert(Solution{}.maximumTime(time) == "09:39");
  }

  {
    string time = "1?:22";
    assert(Solution{}.maximumTime(time) == "19:22");
  }

  return 0;
}