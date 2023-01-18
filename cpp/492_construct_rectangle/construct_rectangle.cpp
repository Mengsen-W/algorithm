/*
 * @Date: 2021-10-23 02:11:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-23 02:19:00
 */

#include <cassert>
#include <cmath>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> constructRectangle(int area) {
    int w = sqrt(1.0 * area);
    while (area % w) {
      --w;
    }
    return {area / w, w};
  }
};


int main() {
  assert(Solution().constructRectangle(4)== move(vector<int>{2,2}));
}