/*
 * @Date: 2022-09-14
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-14
 * @FilePath: /algorithm/1619_trim_mean/trim_mean.cpp
 */

#include <assert.h>

#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  double trimMean(vector<int>& arr) {
    int n = arr.size();
    sort(arr.begin(), arr.end());
    int partialSum = accumulate(arr.begin() + n / 20, arr.begin() + (19 * n / 20), 0);
    return partialSum / (n * 0.9);
  }
};

int main() {
  {
    vector<int> arr{1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3};
    double ans = 2.00000;
    assert(Solution().trimMean(arr) == ans);
  }

  {
    vector<int> arr{6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0};
    double ans = 4.00000;
    assert(Solution().trimMean(arr) == ans);
  }

  {
    vector<int> arr{6, 0, 7, 0, 7, 5, 7, 8, 3,  4, 0, 7, 8, 1, 6, 8,  1, 1, 2, 4,
                    8, 1, 9, 5, 4, 3, 8, 5, 10, 8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4};
    double ans = 4.77777777777777778;
    assert(Solution().trimMean(arr) == ans);
  }

  {
    vector<int> arr{4,  8, 4,  10, 0, 7, 1, 3, 7, 8,  8,  3, 4, 1,  6, 2, 1, 1, 8,  0, 9, 8, 0, 3, 9,  10, 3,
                    10, 1, 10, 7,  3, 2, 1, 4, 9, 10, 7,  6, 4, 0,  8, 5, 1, 2, 1,  6, 2, 5, 0, 7, 10, 9,  10,
                    3,  7, 10, 5,  8, 5, 7, 6, 7, 6,  10, 9, 5, 10, 5, 5, 7, 2, 10, 7, 7, 8, 2, 0, 1,  1};
    double ans = 5.291666666666666667;
    assert(Solution().trimMean(arr) == ans);
  }
}