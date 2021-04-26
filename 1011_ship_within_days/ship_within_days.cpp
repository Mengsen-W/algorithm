/*
 * @Date: 2021-04-26 09:53:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-26 10:38:05
 */

#include <algorithm>
#include <cassert>
#include <iostream>
#include <numeric>
#include <vector>
using namespace std;

int ship_within_days(vector<int> weights, int d) {
  // 确定左右边界最少每天运送货物
  int left = *max_element(weights.begin(), weights.end());
  int right = accumulate(weights.begin(), weights.end(), 0);
  while (left < right) {
    int mid = (right + left) / 2;
    // need 为需要运送的天数
    // cur 为当前这一天已经运送的包裹重量之和
    int need = 1, cur = 0;
    for (int weight : weights) {
      // reset
      if (cur + weight > mid) {
        ++need;
        cur = 0;
      }
      cur += weight;
    }
    if (need <= d)
      right = mid;
    else
      left = mid + 1;
  }
  return left;
}

int main() {
  {
    vector<int> weights{1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    int d = 5;
    assert(ship_within_days(weights, d) == 15);
  }
  {
    vector<int> weights{3, 2, 2, 4, 1, 4};
    int d = 3;
    assert(ship_within_days(weights, d) == 6);
  }
  {
    vector<int> weights{1, 2, 3, 1, 1};
    int d = 4;
    assert(ship_within_days(weights, d) == 3);
  }
}