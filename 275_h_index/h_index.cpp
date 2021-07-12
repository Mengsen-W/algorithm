/*
 * @Date: 2021-07-12 08:06:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-12 08:19:15
 */

#include <cassert>
#include <vector>
using namespace std;

int hIndex(vector<int>& citations) {
  int n = citations.size();
  int left = 0, right = n - 1;
  while (left <= right) {
    int mid = left + (right - left) / 2;
    if (citations[mid] >= n - mid) {
      right = mid - 1;
    } else {
      left = mid + 1;
    }
  }
  return n - left;
}

int main() {
  vector<int> citations{0, 1, 3, 5, 6};
  assert(hIndex(citations) == 3);
}