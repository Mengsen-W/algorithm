/*
 * @Date: 2023-10-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-09
 * @FilePath: /algorithm/cpp/2578_split_num/split_num.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int splitNum(int num) {
    string stnum = to_string(num);
    sort(stnum.begin(), stnum.end());
    int num1 = 0, num2 = 0;
    for (int i = 0; i < stnum.size(); ++i) {
      if (i % 2 == 0) {
        num1 = num1 * 10 + (stnum[i] - '0');
      } else {
        num2 = num2 * 10 + (stnum[i] - '0');
      }
    }
    return num1 + num2;
  }
};

int main() {
  vector<tuple<int, int>> tests{
      {4325, 59},
      {687, 75},
  };

  for (auto& [num, ans] : tests) {
    assert(Solution().splitNum(num) == ans);
  }
}
