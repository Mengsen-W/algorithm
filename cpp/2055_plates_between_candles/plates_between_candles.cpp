/*
 * @Date: 2022-03-08 00:45:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-08 01:10:07
 * @FilePath: /algorithm/2055_plates_between_candles/plates_between_candles.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> platesBetweenCandles(string s, vector<vector<int>> queries) {
    int n = s.length();
    vector<int> preSum(n);
    for (int i = 0, sum = 0; i < n; i++) {
      if (s[i] == '*') {
        sum++;
      }
      preSum[i] = sum;
    }
    vector<int> left(n);
    for (int i = 0, l = -1; i < n; i++) {
      if (s[i] == '|') {
        l = i;
      }
      left[i] = l;
    }
    vector<int> right(n);
    for (int i = n - 1, r = -1; i >= 0; i--) {
      if (s[i] == '|') {
        r = i;
      }
      right[i] = r;
    }
    vector<int> ans;
    for (auto& query : queries) {
      int x = right[query[0]], y = left[query[1]];
      ans.push_back(x == -1 || y == -1 || x >= y ? 0 : preSum[y] - preSum[x]);
    }
    return ans;
  }
};

int main() {
  assert((Solution().platesBetweenCandles("**|**|***|", {{2, 5}, {5, 9}}) ==
          vector<int>{2, 3}));
  assert((Solution().platesBetweenCandles(
              "***|**|*****|**||**|*",
              {{1, 17}, {4, 5}, {14, 17}, {5, 11}, {15, 16}}) ==
          vector<int>{9, 0, 0, 0, 0}));
}