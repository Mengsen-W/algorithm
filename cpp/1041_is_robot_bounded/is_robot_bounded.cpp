/*
 * @Date: 2023-04-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-11
 * @FilePath: /algorithm/cpp/1041_is_robot_bounded/is_robot_bounded.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isRobotBounded(string instructions) {
    vector<vector<int>> direc{{0, 1}, {1, 0}, {0, -1}, {-1, 0}};
    int direcIndex = 0;
    int x = 0, y = 0;
    for (char instruction : instructions) {
      if (instruction == 'G') {
        x += direc[direcIndex][0];
        y += direc[direcIndex][1];
      } else if (instruction == 'L') {
        direcIndex += 3;
        direcIndex %= 4;
      } else {
        direcIndex++;
        direcIndex %= 4;
      }
    }
    return direcIndex != 0 || (x == 0 && y == 0);
  }
};

int main() {
  {
    string instructions{"GGLLGG"};
    assert(Solution().isRobotBounded(instructions));
  }

  {
    string instructions{"GG"};
    assert(!Solution().isRobotBounded(instructions));
  }

  {
    string instructions{"GL"};
    assert(Solution().isRobotBounded(instructions));
  }
}