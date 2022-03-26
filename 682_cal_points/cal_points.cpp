/*
 * @Date: 2022-03-25 23:07:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-25 23:36:55
 * @FilePath: /algorithm/682_cal_points/cal_points.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int calPoints(vector<string>& ops) {
    int ret = 0;
    vector<int> points;
    for (auto& op : ops) {
      int n = points.size();
      switch (op[0]) {
        case '+':
          ret += points[n - 1] + points[n - 2];
          points.push_back(points[n - 1] + points[n - 2]);
          break;
        case 'D':
          ret += 2 * points[n - 1];
          points.push_back(2 * points[n - 1]);
          break;
        case 'C':
          ret -= points[n - 1];
          points.pop_back();
          break;
        default:
          ret += stoi(op);
          points.push_back(stoi(op));
          break;
      }
    }
    return ret;
  }
};

int main() {
  {
    vector<string> ops{"5", "2", "C", "D", "+"};
    assert(Solution().calPoints(ops) == 30);
  }

  {
    vector<string> ops{"5", "-2", "4", "C", "D", "9", "+", "+"};
    assert(Solution().calPoints(ops) == 27);
  }

  {
    vector<string> ops{"1"};
    assert(Solution().calPoints(ops) == 1);
  }
}