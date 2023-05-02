/*
 * @Date: 2023-05-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-02
 * @FilePath: /algorithm/cpp/970_powerful_integers/powerful_integers.cpp
 */

#include <cassert>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> powerfulIntegers(int x, int y, int bound) {
    unordered_set<int> cnt;
    int value1 = 1;
    for (int i = 0; i < 21; i++) {
      int value2 = 1;
      for (int j = 0; j < 21; j++) {
        int value = value1 + value2;
        if (value <= bound) {
          cnt.emplace(value);
        } else {
          break;
        }
        value2 *= y;
      }
      if (value1 > bound) {
        break;
      }
      value1 *= x;
    }
    return vector<int>(cnt.begin(), cnt.end());
  }
};

int main() {
  {
    int x = 2;
    int y = 3;
    int bound = 10;
    vector<int> ans{2, 3, 4, 5, 7, 9, 10};
    assert(Solution().powerfulIntegers(x, y, bound) == ans);
  }

  {
    int x = 3;
    int y = 5;
    int bound = 15;
    vector<int> ans{2, 4, 6, 8, 10, 14};
    assert(Solution().powerfulIntegers(x, y, bound) == ans);
  }
}
