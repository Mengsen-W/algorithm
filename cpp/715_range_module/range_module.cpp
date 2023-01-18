/*
 * @Date: 2022-06-21 09:45:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-21 09:48:50
 * @FilePath: /algorithm/715_range_module/range_module.cpp
 */

#include <cassert>
#include <map>

using namespace std;

class RangeModule {
 public:
  RangeModule() {}

  void addRange(int left, int right) {
    auto it = intervals.upper_bound(left);
    if (it != intervals.begin()) {
      auto start = prev(it);
      if (start->second >= right) {
        return;
      }
      if (start->second >= left) {
        left = start->first;
        intervals.erase(start);
      }
    }
    while (it != intervals.end() && it->first <= right) {
      right = max(right, it->second);
      it = intervals.erase(it);
    }
    intervals[left] = right;
  }

  bool queryRange(int left, int right) {
    auto it = intervals.upper_bound(left);
    if (it == intervals.begin()) {
      return false;
    }
    it = prev(it);
    return right <= it->second;
  }

  void removeRange(int left, int right) {
    auto it = intervals.upper_bound(left);
    if (it != intervals.begin()) {
      auto start = prev(it);
      if (start->second >= right) {
        int ri = start->second;
        if (start->first == left) {
          intervals.erase(start);
        } else {
          start->second = left;
        }
        if (right != ri) {
          intervals[right] = ri;
        }
        return;
      } else if (start->second > left) {
        start->second = left;
      }
    }
    while (it != intervals.end() && it->first < right) {
      if (it->second <= right) {
        it = intervals.erase(it);
      } else {
        intervals[right] = it->second;
        intervals.erase(it);
        break;
      }
    }
  }

 private:
  map<int, int> intervals;
};

int main() {
  RangeModule r{};
  r.addRange(10, 20);
  r.removeRange(14, 16);
  assert(r.queryRange(10, 14));
  assert(!r.queryRange(13, 15));
  assert(r.queryRange(16, 17));
}