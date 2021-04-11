/*
 * @Date: 2021-04-11 09:55:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-11 10:03:54
 */

#include <cassert>
#include <vector>

using namespace std;

int nth_ugly_number(int n) {
  vector<int> ugly(n, 1), idx(3, 0);
  for (int i = 1; i < n; ++i) {
    int a = ugly[idx[0]] * 2, b = ugly[idx[1]] * 3, c = ugly[idx[2]] * 5;
    int next = std::min(a, std::min(b, c));
    if (next == a) {
      ++idx[0];
    }
    if (next == b) {
      ++idx[1];
    }
    if (next == c) {
      ++idx[2];
    }
    ugly[i] = next;
  }
  return ugly.back();
}

int main() {
  assert(nth_ugly_number(10) == 12);
  assert(nth_ugly_number(1) == 1);
  return 0;
}