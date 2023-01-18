/*
 * @Date: 2022-12-31
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-31
 * @FilePath: /algorithm/2037_min_moves_to_seat/min_moves_to_seat.cpp
 */

#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minMovesToSeat(vector<int>& seats, vector<int>& students) {
    sort(seats.begin(), seats.end());
    sort(students.begin(), students.end());
    int res = 0;
    for (int i = 0; i < seats.size(); i++) {
      res += abs(seats[i] - students[i]);
    }
    return res;
  }
};

int main() {
  {
    vector<int> seats{3, 1, 5};
    vector<int> students{2, 7, 4};
    int ans = 4;
    assert(Solution().minMovesToSeat(seats, students) == ans);
  }

  {
    vector<int> seats{4, 1, 5, 9};
    vector<int> students{1, 3, 2, 6};
    int ans = 7;
    assert(Solution().minMovesToSeat(seats, students) == ans);
  }

  {
    vector<int> seats{2, 2, 6, 6};
    vector<int> students{1, 3, 2, 6};
    int ans = 4;
    assert(Solution().minMovesToSeat(seats, students) == ans);
  }
}