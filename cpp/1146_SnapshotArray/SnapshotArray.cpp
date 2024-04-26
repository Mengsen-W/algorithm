/*
 * @Date: 2024-04-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-26
 * @FilePath: /algorithm/cpp/1146_SnapshotArray/SnapshotArray.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class SnapshotArray {
 public:
  SnapshotArray(int length) : snap_cnt(0), data(length) {}

  void set(int index, int val) { data[index].emplace_back(snap_cnt, val); }

  int snap() { return snap_cnt++; }

  int get(int index, int snap_id) {
    auto x = upper_bound(data[index].begin(), data[index].end(), pair{snap_id + 1, -1});
    return x == data[index].begin() ? 0 : prev(x)->second;
  }

 private:
  int snap_cnt;
  vector<vector<pair<int, int>>> data;
};

int main() {
  SnapshotArray snapshotArray(3);
  snapshotArray.set(0, 5);
  assert(snapshotArray.snap() == 0);
  snapshotArray.set(0, 6);
  assert(snapshotArray.get(0, 0) == 5);
  return 0;
}