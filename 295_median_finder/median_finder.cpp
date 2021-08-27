/*
 * @Date: 2021-08-27 18:06:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-27 18:14:39
 */

#include <cassert>
#include <cstddef>
#include <set>

using namespace std;

class MedianFinder {
  multiset<int> nums;
  multiset<int>::iterator left, right;

 public:
  MedianFinder() : left(nums.end()), right(nums.end()) {}

  void addNum(int num) {
    const size_t n = nums.size();

    nums.insert(num);
    if (!n) {
      left = right = nums.begin();
    } else if (n & 1) {
      if (num < *left) {
        left--;
      } else {
        right++;
      }
    } else {
      if (num > *left && num < *right) {
        left++;
        right--;
      } else if (num >= *right) {
        left++;
      } else {
        right--;
        left = right;
      }
    }
  }

  double findMedian() { return (*left + *right) / 2.0; }
};

int main() {
  MedianFinder* obj = new MedianFinder();
  obj->addNum(1);
  obj->addNum(2);
  assert(obj->findMedian() == 1.5);
  obj->addNum(3);
  assert(obj->findMedian() == 2);
}