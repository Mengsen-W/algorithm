/*
 * @Date: 2023-12-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-25
 * @FilePath: /algorithm/cpp/1276_num_of_burgers/num_of_burgers.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> numOfBurgers(int tomatoSlices, int cheeseSlices) {
    if (tomatoSlices % 2 != 0 || tomatoSlices < cheeseSlices * 2 || cheeseSlices * 4 < tomatoSlices) {
      return {};
    }
    return {tomatoSlices / 2 - cheeseSlices, cheeseSlices * 2 - tomatoSlices / 2};
  }
};

int main() {
  vector<tuple<int, int, vector<int>>> tests{
      {16, 7, {1, 6}}, {17, 4, {}}, {4, 17, {}}, {0, 0, {0, 0}}, {2, 1, {0, 1}},
  };

  for (auto &[tomatoSlices, cheeseSlices, ans] : tests) {
    assert(Solution().numOfBurgers(tomatoSlices, cheeseSlices) == ans);
  }
}