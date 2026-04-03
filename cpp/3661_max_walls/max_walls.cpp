#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxWalls(vector<int>& robots, vector<int>& distance, vector<int>& walls) {
    int n = robots.size();
    vector<pair<int, int>> robotDist;
    for (int i = 0; i < n; i++) {
      robotDist.push_back({robots[i], distance[i]});
    }
    sort(robotDist.begin(), robotDist.end());
    sort(walls.begin(), walls.end());

    int m = walls.size();
    int rightPtr = 0, leftPtr = 0, curPtr = 0, robotPtr = 0;

    // DP 变量：只保存前一个值
    int subLeft = 0, subRight = 0, prevRight = 0;

    for (int i = 0; i < n; i++) {
      int robotPos = robotDist[i].first;
      int robotDistVal = robotDist[i].second;

      // rightPtr 找到第一个 > robotPos 的位置 (对应 upper_bound)
      while (rightPtr < m && walls[rightPtr] <= robotPos) {
        rightPtr++;
      }
      int pos1 = rightPtr;

      // curPtr 找到第一个 >= robotPos 的位置 (对应 lower_bound)
      while (curPtr < m && walls[curPtr] < robotPos) {
        curPtr++;
      }
      int pos2 = curPtr;

      // leftPtr 找到第一个 >= 左边界的墙
      int leftBound = (i >= 1) ? max(robotPos - robotDistVal, robotDist[i - 1].first + 1) : robotPos - robotDistVal;
      while (leftPtr < m && walls[leftPtr] < leftBound) {
        leftPtr++;
      }
      int leftPos = leftPtr;
      int currentLeft = pos1 - leftPos;

      // 计算右边可达的墙的数量
      int rightBound = (i < n - 1) ? min(robotPos + robotDistVal, robotDist[i + 1].first - 1) : robotPos + robotDistVal;
      while (rightPtr < m && walls[rightPtr] <= rightBound) {
        rightPtr++;
      }
      int rightPos = rightPtr;
      int currentRight = rightPos - pos2;

      // robotPtr 找到上一个机器人的位置
      int currentNum = 0;
      if (i > 0) {
        while (robotPtr < m && walls[robotPtr] < robotDist[i - 1].first) {
          robotPtr++;
        }
        int pos3 = robotPtr;
        currentNum = pos1 - pos3;
      }

      if (i == 0) {
        subLeft = currentLeft;
        subRight = currentRight;
      } else {
        int newsubLeft = max(subLeft + currentLeft, subRight - prevRight + min(currentLeft + prevRight, currentNum));
        int newsubRight = max(subLeft + currentRight, subRight + currentRight);
        subLeft = newsubLeft;
        subRight = newsubRight;
      }

      prevRight = currentRight;
    }

    return max(subLeft, subRight);
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, vector<int>, int>> tests{
      {{4}, {3}, {1, 10}, 1},
      {{10, 2}, {5, 1}, {5, 2, 7}, 3},
      {{1, 2}, {100, 1}, {10}, 0},
  };

  for (auto& [robots, distance, walls, expected] : tests) {
    assert(Solution().maxWalls(robots, distance, walls) == expected);
  }
}