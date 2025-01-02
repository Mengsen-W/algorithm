#include <cassert>
#include <unordered_set>

using namespace std;

class MyCalendar {
  unordered_set<int> tree, lazy;

 public:
  bool query(int start, int end, int l, int r, int idx) {
    if (r < start || end < l) {
      return false;
    }
    /* 如果该区间已被预订，则直接返回 */
    if (lazy.count(idx)) {
      return true;
    }
    if (start <= l && r <= end) {
      return tree.count(idx);
    }
    int mid = (l + r) >> 1;
    return query(start, end, l, mid, 2 * idx) || query(start, end, mid + 1, r, 2 * idx + 1);
  }

  void update(int start, int end, int l, int r, int idx) {
    if (r < start || end < l) {
      return;
    }
    if (start <= l && r <= end) {
      tree.emplace(idx);
      lazy.emplace(idx);
    } else {
      int mid = (l + r) >> 1;
      update(start, end, l, mid, 2 * idx);
      update(start, end, mid + 1, r, 2 * idx + 1);
      tree.emplace(idx);
      if (lazy.count(2 * idx) && lazy.count(2 * idx + 1)) {
        lazy.emplace(idx);
      }
    }
  }

  bool book(int start, int end) {
    if (query(start, end - 1, 0, 1e9, 1)) {
      return false;
    }
    update(start, end - 1, 0, 1e9, 1);
    return true;
  }
};

int main() {
  MyCalendar *myCalendar = new MyCalendar();
  assert(myCalendar->book(10, 20) == true);
  assert(myCalendar->book(15, 25) == false);
  assert(myCalendar->book(20, 30) == true);
}