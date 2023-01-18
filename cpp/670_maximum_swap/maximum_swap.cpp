/*
 * @Date: 2022-09-13
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-13
 * @FilePath: /algorithm/670_maximum_swap/maximum_swap.cpp
 */

#include <assert.h>

#include <string>

using namespace std;

class Solution {
 public:
  int maximumSwap(int num) {
    string charArray = to_string(num);
    int n = charArray.size();
    int maxIdx = n - 1;
    int idx1 = -1, idx2 = -1;
    for (int i = n - 1; i >= 0; i--) {
      if (charArray[i] > charArray[maxIdx]) {
        maxIdx = i;
      } else if (charArray[i] < charArray[maxIdx]) {
        idx1 = i;
        idx2 = maxIdx;
      }
    }
    if (idx1 >= 0) {
      swap(charArray[idx1], charArray[idx2]);
      return stoi(charArray);
    } else {
      return num;
    }
  }
};

int main() {
  assert(Solution().maximumSwap(2736) == 7236);
  assert(Solution().maximumSwap(9973) == 9973);
}