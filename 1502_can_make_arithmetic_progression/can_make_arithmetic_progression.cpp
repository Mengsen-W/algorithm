/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-18 09:45:30
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-18 09:48:07
 */

#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

bool can_make_arithmetic_progression(vector<int>& arr) {
  sort(arr.begin(), arr.end());
  const int diff = arr[1] - arr[0];
  int len = arr.size();
  for (int i = 2; i < len; ++i) {
    if (arr[i] - arr[i - 1] != diff) return false;
  }
  return true;
}

int main(void) {
  vector<int> arr;
  arr = {3, 5, 1};
  assert(can_make_arithmetic_progression(arr));
  arr = {2, 4, 1};
  assert(!can_make_arithmetic_progression(arr));
}