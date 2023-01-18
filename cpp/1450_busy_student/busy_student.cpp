/*
 * @Date: 2022-08-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-19
 * @FilePath: /algorithm/1450_busy_student/busy_student.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int busyStudent(vector<int>& startTime, vector<int>& endTime, int queryTime) {
    sort(startTime.begin(), startTime.end());
    sort(endTime.begin(), endTime.end());
    int lessStart = upper_bound(startTime.begin(), startTime.end(), queryTime) - startTime.begin();
    int lessEnd = lower_bound(endTime.begin(), endTime.end(), queryTime) - endTime.begin();
    return lessStart - lessEnd;
  }
};

int main() {
  {
    vector<int> startTime{1, 2, 3};
    vector<int> endTime{3, 2, 7};
    int queryTime = 4;
    assert(Solution().busyStudent(startTime, endTime, queryTime) == 1);
  }

  {
    vector<int> startTime{4};
    vector<int> endTime{4};
    int queryTime = 4;
    assert(Solution().busyStudent(startTime, endTime, queryTime) == 1);
  }

  {
    vector<int> startTime{1, 1, 1, 1};
    vector<int> endTime{1, 3, 2, 4};
    int queryTime = 7;
    assert(Solution().busyStudent(startTime, endTime, queryTime) == 0);
  }

  {
    vector<int> startTime{9, 8, 7, 6, 5, 4, 3, 2, 1};
    vector<int> endTime{10, 10, 10, 10, 10, 10, 10, 10, 10};
    int queryTime = 5;
    assert(Solution().busyStudent(startTime, endTime, queryTime) == 5);
  }

  {
    vector<int> startTime{1, 2, 3};
    vector<int> endTime{3, 2, 7};
    int queryTime = 4;
    assert(Solution().busyStudent(startTime, endTime, queryTime) == 1);
  }
}