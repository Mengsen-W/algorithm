/*
 * @Date: 2022-06-06 09:37:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-06 09:51:00
 * @FilePath: /algorithm/732_my_calendar_three/my_calendar_three.cpp
 */

#include <cassert>
#include <unordered_map>

using namespace std;

class MyCalendarThree {
 public:
  unordered_map<int, pair<int, int>> tree;

  MyCalendarThree() {}

  void update(int start, int end, int l, int r, int idx) {
    if (r < start || end < l) {
      return;
    }
    if (start <= l && r <= end) {
      tree[idx].first++;
      tree[idx].second++;
    } else {
      int mid = (l + r) >> 1;
      update(start, end, l, mid, 2 * idx);
      update(start, end, mid + 1, r, 2 * idx + 1);
      tree[idx].first = tree[idx].second + max(tree[2 * idx].first, tree[2 * idx + 1].first);
    }
  }

  int book(int start, int end) {
    update(start, end - 1, 0, 1e9, 1);
    return tree[1].first;
  }
};

int main() {
  MyCalendarThree my_calendar_three{};
  assert(my_calendar_three.book(10, 20) == 1);
  assert(my_calendar_three.book(50, 60) == 1);
  assert(my_calendar_three.book(10, 40) == 2);
  assert(my_calendar_three.book(5, 15) == 3);
  assert(my_calendar_three.book(5, 10) == 3);
  assert(my_calendar_three.book(25, 55) == 3);
}