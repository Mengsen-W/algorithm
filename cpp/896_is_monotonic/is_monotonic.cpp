/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-28 14:59:29
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-28 15:10:43
 */

#include <cassert>
#include <vector>

using namespace std;

bool is_monotonic(vector<int> &a) {
  bool inc = true, dec = true;
  int n = a.size();
  for (int i = 0; i < n - 1; ++i) {
    if (a[i] > a[i + 1]) inc = false;
    if (a[i] < a[i + 1]) dec = false;
  }
  return inc | dec;
}

int main() {
  vector<int> a{};
  a = {1, 2, 2, 3};
  assert(is_monotonic(a));
  a = {6, 5, 4, 4};
  assert(is_monotonic(a));
  a = {1, 3, 2};
  assert(!is_monotonic(a));
  a = {1, 2, 4, 5};
  assert(is_monotonic(a));
  a = {1, 1, 1};
  assert(is_monotonic(a));
  return 0;
}