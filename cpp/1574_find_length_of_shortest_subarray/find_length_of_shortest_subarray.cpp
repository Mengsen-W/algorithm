/*
 * @Date: 2023-03-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-25
 * @FilePath: /algorithm/cpp/1574_find_length_of_shortest_subarray/find_length_of_shortest_subarray.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int findLengthOfShortestSubarray(vector<int>& arr) {
    int n = arr.size(), j = n - 1;
    while (j > 0 && arr[j - 1] <= arr[j]) {
      j--;
    }
    if (j == 0) {
      return 0;
    }
    int res = j;
    for (int i = 0; i < n; i++) {
      while (j < n && arr[j] < arr[i]) {
        j++;
      }
      res = min(res, j - i - 1);
      if (i + 1 < n && arr[i] > arr[i + 1]) {
        break;
      }
    }
    return res;
  }
};

int main() {
  {
    vector<int> arr{1, 2, 3, 10, 4, 2, 3, 5};
    int ans = 3;
    assert(Solution().findLengthOfShortestSubarray(arr) == ans);
  }

  {
    vector<int> arr{5, 4, 3, 2, 1};
    int ans = 4;
    assert(Solution().findLengthOfShortestSubarray(arr) == ans);
  }

  {
    vector<int> arr{1, 2, 3};
    int ans = 0;
    assert(Solution().findLengthOfShortestSubarray(arr) == ans);
  }

  {
    vector<int> arr{1};
    int ans = 0;
    assert(Solution().findLengthOfShortestSubarray(arr) == ans);
  }
}