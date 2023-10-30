/*
 * @Date: 2021-07-12 08:06:39
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-30
 */

#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int hIndex(vector<int>& citations) {
    int n = citations.size();
    int left = 0, right = n - 1;
    while (left <= right) {
      int mid = left + (right - left) / 2;
      if (citations[mid] >= n - mid) {
        right = mid - 1;
      } else {
        left = mid + 1;
      }
    }
    return n - left;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{0, 1, 3, 5, 6}, 3},
      {{1, 2, 100}, 2},
  };

  for (auto& [citations, ans] : tests) {
    assert(Solution().hIndex(citations) == ans);
  }
}