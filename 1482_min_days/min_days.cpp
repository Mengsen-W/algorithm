/*
 * @Date: 2021-05-09 08:52:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-09 09:12:23
 * @FilePath: \algorithm\1482_min_days\min_days.cpp
 * @Description: file content
 */

#include <cassert>
#include <vector>
using namespace std;

bool can_make(vector<int>& bloomDay, int days, int m, int k) {
  int bouquets = 0;
  int flowers = 0;
  int length = bloomDay.size();
  for (int i = 0; i < length && bouquets < m; i++) {
    if (bloomDay[i] <= days) {
      flowers++;
      if (flowers == k) {
        bouquets++;
        flowers = 0;
      }
    } else {
      flowers = 0;
    }
  }
  return bouquets >= m;
}

int min_days(vector<int>& bloomDay, int m, int k) {
  if (k * m > bloomDay.size()) {
    return -1;
  }
  int low = 1, high = 1;
  int length = bloomDay.size();
  for (int i = 0; i < length; i++) {
    high = max(high, bloomDay[i]);
  }
  while (low < high) {
    int days = (high - low) / 2 + low;
    if (can_make(bloomDay, days, m, k)) {
      high = days;
    } else {
      low = days + 1;
    }
  }
  return low;
}

int main() {
  {
    vector<int> bloom_day{1, 10, 3, 10, 2};
    assert(min_days(bloom_day, 3, 1) == 3);
  }
  {
    vector<int> bloom_day{1, 10, 3, 10, 2};
    assert(min_days(bloom_day, 3, 2) == -1);
  }
  {
    vector<int> bloom_day{7, 7, 7, 7, 12, 7, 7};
    assert(min_days(bloom_day, 2, 3) == 12);
  }
  {
    vector<int> bloom_day{1000000000, 1000000000};
    assert(min_days(bloom_day, 1, 1) == 1000000000);
  }
  {
    vector<int> bloom_day{1, 10, 2, 9, 3, 8, 4, 7, 5, 6};
    assert(min_days(bloom_day, 4, 2) == 9);
  }
}