/*
 * @Date: 2021-05-18 08:23:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-18 08:40:14
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

int countTriplets(const vector<int> &arr) {
  int n = arr.size();
  unordered_map<int, int> cnt, total;
  int ans = 0, s = 0;
  for (int k = 0; k < n; ++k) {
    int val = arr[k];
    if (cnt.count(s ^ val)) {
      ans += cnt[s ^ val] * k - total[s ^ val];
    }
    ++cnt[s];
    total[s] += k;
    s ^= val;
  }
  return ans;
}

int main() {
  assert(countTriplets(vector<int>{2, 3, 1, 6, 7}) == 4);
  assert(countTriplets(vector<int>{1, 1, 1, 1, 1}) == 10);
  assert(countTriplets(vector<int>{2, 3}) == 0);
  assert(countTriplets(vector<int>{1, 3, 5, 7, 9}) == 3);
  assert(countTriplets(vector<int>{7, 11, 12, 9, 5, 2, 7, 17, 22}) == 8);
  return 0;
}