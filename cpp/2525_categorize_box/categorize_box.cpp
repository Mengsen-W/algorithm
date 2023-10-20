/*
 * @Date: 2023-10-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-20
 * @FilePath: /algorithm/cpp/2525_categorize_box/categorize_box.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string categorizeBox(int length, int width, int height, int mass) {
    long long maxd = max(length, max(width, height)), vol = 1L * length * width * height;
    bool isBulky = maxd >= 10000 || vol >= 1000000000, isHeavy = mass >= 100;
    if (isBulky && isHeavy) {
      return "Both";
    } else if (isBulky) {
      return "Bulky";
    } else if (isHeavy) {
      return "Heavy";
    } else {
      return "Neither";
    }
  }
};

int main() {
  vector<tuple<int, int, int, int, string>> tests{
      {1000, 35, 700, 300, "Heavy"},
      {200, 50, 800, 50, "Neither"},
  };

  for (auto &[length, width, height, mass, ans] : tests) {
    assert(Solution().categorizeBox(length, width, height, mass) == ans);
  }
}