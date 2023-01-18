/*
 * @Date: 2022-11-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-15
 * @FilePath: /algorithm/507_maximum_units/maximum_units.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumUnits(vector<vector<int>> &boxTypes, int truckSize) {
    sort(boxTypes.begin(), boxTypes.end(), [](const vector<int> &a, const vector<int> &b) { return a[1] > b[1]; });
    int res = 0;
    for (auto &boxType : boxTypes) {
      int numberOfBoxes = boxType[0];
      int numberOfUnitsPerBox = boxType[1];
      if (numberOfBoxes < truckSize) {
        res += numberOfBoxes * numberOfUnitsPerBox;
        truckSize -= numberOfBoxes;
      } else {
        res += truckSize * numberOfUnitsPerBox;
        break;
      }
    }
    return res;
  }
};

int main() {
  {
    vector<vector<int>> boxTypes{{1, 3}, {2, 2}, {3, 1}};
    int truckSize = 4;
    int ans = 8;
    assert(Solution().maximumUnits(boxTypes, truckSize) == ans);
  }

  {
    vector<vector<int>> boxTypes{{5, 10}, {2, 5}, {4, 7}, {3, 9}};
    int truckSize = 10;
    int ans = 91;
    assert(Solution().maximumUnits(boxTypes, truckSize) == ans);
  }
}