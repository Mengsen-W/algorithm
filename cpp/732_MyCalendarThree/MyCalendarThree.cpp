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
  MyCalendarThree *myCalendarThree = new MyCalendarThree();
  assert(myCalendarThree->book(10, 20) == 1);
  assert(myCalendarThree->book(50, 60) == 1);
  assert(myCalendarThree->book(10, 40) == 2);
  assert(myCalendarThree->book(5, 15) == 3);
  assert(myCalendarThree->book(5, 10) == 3);
  assert(myCalendarThree->book(25, 55) == 3);
}