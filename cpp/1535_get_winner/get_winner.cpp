/*
 * @Date: 2024-05-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-19
 * @FilePath: /algorithm/cpp/1535_get_winner/get_winner.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int getWinner(vector<int>& arr, int k) {
    int prev = max(arr[0], arr[1]);
    if (k == 1) {
      return prev;
    }
    int consecutive = 1;
    int maxNum = prev;
    int length = arr.size();
    for (int i = 2; i < length; i++) {
      int curr = arr[i];
      if (prev > curr) {
        consecutive++;
        if (consecutive == k) {
          return prev;
        }
      } else {
        prev = curr;
        consecutive = 1;
      }
      maxNum = max(maxNum, curr);
    }
    return maxNum;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{2, 1, 3, 5, 4, 6, 7}, 2, 5},
      {{3, 2, 1}, 10, 3},
      {{1, 9, 8, 2, 3, 7, 6, 4, 5}, 7, 9},
      {{1, 11, 22, 33, 44, 55, 66, 77, 88, 99}, 1000000000, 99},
  };

  for (auto& [arr, k, ans] : tests) {
    assert(Solution().getWinner(arr, k) == ans);
  }
}