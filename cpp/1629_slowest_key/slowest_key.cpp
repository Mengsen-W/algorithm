/*
 * @Date: 2022-01-09 01:04:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-09 01:14:30
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  char slowestKey(vector<int> releaseTimes, string keysPressed) {
    int n = releaseTimes.size();
    char ans = keysPressed[0];
    int maxTime = releaseTimes[0];
    for (int i = 1; i < n; i++) {
      char key = keysPressed[i];
      int time = releaseTimes[i] - releaseTimes[i - 1];
      if (time > maxTime || (time == maxTime && key > ans)) {
        ans = key;
        maxTime = time;
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().slowestKey({9, 29, 49, 50}, "cbcd") == 'c');
  assert(Solution().slowestKey({12, 23, 36, 46, 62}, "supda") == 'a');
}