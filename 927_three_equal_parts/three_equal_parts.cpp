/*
 * @Date: 2022-10-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-06
 * @FilePath: /algorithm/927_three_equal_parts/three_equal_parts.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> threeEqualParts(vector<int>& arr) {
    int sum = accumulate(arr.begin(), arr.end(), 0);
    if (sum % 3 != 0) {
      return {-1, -1};
    }
    if (sum == 0) {
      return {0, 2};
    }

    int partial = sum / 3;
    int first = 0, second = 0, third = 0, cur = 0;
    for (int i = 0; i < arr.size(); i++) {
      if (arr[i] == 1) {
        if (cur == 0) {
          first = i;
        } else if (cur == partial) {
          second = i;
        } else if (cur == 2 * partial) {
          third = i;
        }
        cur++;
      }
    }

    int len = (int)arr.size() - third;
    if (first + len <= second && second + len <= third) {
      int i = 0;
      while (third + i < arr.size()) {
        if (arr[first + i] != arr[second + i] || arr[first + i] != arr[third + i]) {
          return {-1, -1};
        }
        i++;
      }
      return {first + len - 1, second + len};
    }
    return {-1, -1};
  }
};

int main() {
  {
    vector<int> arr{1, 0, 1, 0, 1};
    vector<int> ans{0, 3};
    assert(Solution().threeEqualParts(arr) == ans);
  }

  {
    vector<int> arr{1, 1, 0, 1, 1};
    vector<int> ans{-1, -1};
    assert(Solution().threeEqualParts(arr) == ans);
  }

  {
    vector<int> arr{1, 1, 0, 0, 1};
    vector<int> ans{0, 2};
    assert(Solution().threeEqualParts(arr) == ans);
  }
}