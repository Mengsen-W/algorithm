/*
 * @Date: 2022-07-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-22
 * @FilePath: /algorithm/757_intersection_size_two/intersection_size_two.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  void help(vector<vector<int>>& intervals, vector<vector<int>>& temp, int pos, int num) {
    for (int i = pos; i >= 0; i--) {
      if (intervals[i][1] < num) {
        break;
      }
      temp[i].push_back(num);
    }
  }

  int intersectionSizeTwo(vector<vector<int>>& intervals) {
    int n = intervals.size();
    int res = 0;
    int m = 2;
    sort(intervals.begin(), intervals.end(), [&](vector<int>& a, vector<int>& b) {
      if (a[0] == b[0]) {
        return a[1] > b[1];
      }
      return a[0] < b[0];
    });
    vector<vector<int>> temp(n);
    for (int i = n - 1; i >= 0; i--) {
      for (int j = intervals[i][0], k = temp[i].size(); k < m; j++, k++) {
        res++;
        help(intervals, temp, i - 1, j);
      }
    }
    return res;
  }
};

int main() {
  {
    vector<vector<int>> intervals{{1, 3}, {1, 4}, {2, 5}, {3, 5}};
    assert(Solution().intersectionSizeTwo(intervals) == 3);
  }

  {
    vector<vector<int>> intervals{{1, 2}, {2, 3}, {2, 4}, {4, 5}};
    assert(Solution().intersectionSizeTwo(intervals) == 5);
  }
}