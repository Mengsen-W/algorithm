/*
 * @Date: 2022-03-31 22:12:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-31 22:22:08
 * @FilePath: /algorithm/954_can_reorder_doubled/can_reorder_doubled.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  bool canReorderDoubled(vector<int> arr) {
    unordered_map<int, int> cnt;
    for (int x : arr) {
      ++cnt[x];
    }
    if (cnt[0] % 2) {
      return false;
    }

    vector<int> vals;
    vals.reserve(cnt.size());
    for (auto &[x, _] : cnt) {
      vals.push_back(x);
    }
    sort(vals.begin(), vals.end(),
         [](int a, int b) { return abs(a) < abs(b); });

    for (int x : vals) {
      if (cnt[2 * x] < cnt[x]) {  // 无法找到足够的 2x 与 x 配对
        return false;
      }
      cnt[2 * x] -= cnt[x];
    }
    return true;
  }
};

int main() {
  assert(Solution().canReorderDoubled(vector<int>{3, 1, 3, 6}) == false);
  assert(Solution().canReorderDoubled(vector<int>{2, 1, 2, 6}) == false);
  assert(Solution().canReorderDoubled(vector<int>{4, -2, 2, -4}) == true);
}