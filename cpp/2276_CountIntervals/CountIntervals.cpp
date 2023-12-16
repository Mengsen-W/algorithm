/*
 * @Date: 2023-12-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-16
 * @FilePath: /algorithm/cpp/2276_CountIntervals/CountIntervals.cpp
 */

#include <cassert>
#include <map>

using namespace std;

class CountIntervals {
 public:
  CountIntervals() {}

  void add(int left, int right) {
    auto interval = mp.upper_bound(right);
    if (interval != mp.begin()) {
      interval--;
    }
    while (interval != mp.end() && interval->first <= right && interval->second >= left) {
      int l = interval->first, r = interval->second;
      left = min(left, l);
      right = max(right, r);
      cnt -= r - l + 1;
      mp.erase(interval);
      interval = mp.upper_bound(right);
      if (interval != mp.begin()) {
        interval--;
      }
    }
    cnt += (right - left + 1);
    mp[left] = right;
  }

  int count() { return cnt; }

 private:
  int cnt = 0;
  map<int, int> mp;
};

int main() {
  CountIntervals countIntervals = CountIntervals();
  countIntervals.add(2, 3);
  countIntervals.add(7, 10);
  assert(countIntervals.count() == 6);
  countIntervals.add(5, 8);
  assert(countIntervals.count() == 8);
}