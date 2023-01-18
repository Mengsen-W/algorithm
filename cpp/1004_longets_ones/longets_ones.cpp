/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-19 09:02:25
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-19 09:05:03
 */

#include <cassert>
#include <vector>

using namespace std;

int longest_ones(vector<int>& a, int k) {
  int count = 0;  // 0 numbers
  int left = 0, right = 0, len = a.size();

  // 在遇到条件破坏的时候同时向右移动，直到条件满足才又继续扩张
  // 向右移动不会产生比现在大的窗口
  while (right < len) {
    if (!a[right]) ++count;
    ++right;
    if (count > k) {
      if (!a[left]) --count;
      ++left;
    }
    // std::cout << "left = " << left << ", " << "right = " << right << ", " <<
    // "count = " << count << std::endl;
  }
  return right - left;
}

int main(void) {
  vector<int> a{};
  a = {1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0};
  assert(longest_ones(a, 2) == 6);
  a = {0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1};
  assert(longest_ones(a, 3) == 10);
  return 0;
}