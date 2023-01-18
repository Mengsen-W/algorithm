/*
 * @Date: 2022-01-26 01:44:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-26 02:18:33
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class DetectSquares {
 public:
  unordered_map<int, unordered_map<int, int>> cnt;
  DetectSquares() {}

  void add(vector<int> point) {
    int x = point[0], y = point[1];
    cnt[y][x]++;
  }

  int count(vector<int> point) {
    int res = 0;
    int x = point[0], y = point[1];
    if (!cnt.count(y)) {
      return 0;
    }
    unordered_map<int, int>& yCnt = cnt[y];
    for (auto& [col, colCnt] : cnt) {
      if (col != y) {
        // 根据对称性，这里可以不用取绝对值
        int d = col - y;
        res += (colCnt.count(x) ? colCnt[x] : 0) *
               (yCnt.count(x + d) ? yCnt[x + d] : 0) *
               (colCnt.count(x + d) ? colCnt[x + d] : 0);
        res += (colCnt.count(x) ? colCnt[x] : 0) *
               (yCnt.count(x - d) ? yCnt[x - d] : 0) *
               (colCnt.count(x - d) ? colCnt[x - d] : 0);
      }
    }
    return res;
  }
};

int main() {
  DetectSquares ds;
  ds.add({3, 10});
  ds.add({11, 2});
  ds.add({3, 2});
  assert(ds.count({11, 10}) == 1);
  assert(ds.count({14, 8}) == 0);
  ds.add({11, 2});
  assert(ds.count({11, 10}) == 2);
  return 0;
}