/*
 * @Date: 2021-07-12 08:24:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-12 08:26:00
 */

#include <cassert>
#include <vector>
using namespace std;

int hIndex(vector<int>& citations) {
  int n = citations.size(), tot = 0;
  vector<int> counter(n + 1);
  for (int i = 0; i < n; i++) {
    if (citations[i] >= n) {
      counter[n]++;
    } else {
      counter[citations[i]]++;
    }
  }
  for (int i = n; i >= 0; i--) {
    tot += counter[i];
    if (tot >= i) {
      return i;
    }
  }
  return 0;
}

int main() {
  vector<int> citations{0, 1, 3, 5, 6};
  assert(hIndex(citations) == 3);
}