/*
 * @Date: 2022-09-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-22
 * @FilePath: /algorithm/cpp/670_maximum_swap/maximum_swap.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

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
  vector<tuple<int, int>> tests{
      {2736, 7236},
      {9973, 9973},
  };

  for (auto& [num, ans] : tests) {
    assert(Solution().maximumSwap(num) == ans);
  }
}