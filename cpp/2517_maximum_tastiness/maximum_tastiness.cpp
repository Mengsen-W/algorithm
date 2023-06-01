/*
 * @Date: 2023-06-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-01
 * @FilePath: /algorithm/cpp/2517_maximum_tastiness/maximum_tastiness.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumTastiness(vector<int>& price, int k) {
    int n = price.size();
    sort(price.begin(), price.end());
    int left = 0, right = price[n - 1] - price[0];
    while (left < right) {
      int mid = (left + right + 1) >> 1;
      if (check(price, k, mid)) {
        left = mid;
      } else {
        right = mid - 1;
      }
    }
    return left;
  }

  bool check(const vector<int>& price, int k, int tastiness) {
    int prev = INT_MIN >> 1;
    int cnt = 0;
    for (int p : price) {
      if (p - prev >= tastiness) {
        cnt++;
        prev = p;
      }
    }
    return cnt >= k;
  }
};

int main() {
  {
    vector<int> price = {13, 5, 1, 8, 21, 2};
    int k = 3;
    int ans = 8;
    assert(Solution().maximumTastiness(price, k) == ans);
  }

  {
    vector<int> price = {1, 3, 1};
    int k = 2;
    int ans = 2;
    assert(Solution().maximumTastiness(price, k) == ans);
  }

  {
    vector<int> price = {7, 7, 7, 7};
    int k = 2;
    int ans = 0;
    assert(Solution().maximumTastiness(price, k) == ans);
  }
}