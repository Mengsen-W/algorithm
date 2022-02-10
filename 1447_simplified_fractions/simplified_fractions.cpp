/*
 * @Date: 2022-02-10 00:35:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-10 00:46:18
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> simplifiedFractions(int n) {
    vector<string> ans;
    for (int denominator = 2; denominator <= n; ++denominator) {
      for (int numerator = 1; numerator < denominator; ++numerator) {
        if (__gcd(numerator, denominator) == 1) {
          ans.emplace_back(to_string(numerator) + "/" + to_string(denominator));
        }
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().simplifiedFractions(2) == vector<string>({"1/2"}));
  assert(Solution().simplifiedFractions(3) ==
         vector<string>({"1/2", "1/3", "2/3"}));
  assert(Solution().simplifiedFractions(4) ==
         vector<string>({"1/2", "1/3", "2/3", "1/4", "3/4"}));
  assert(Solution().simplifiedFractions(1) == vector<string>());
}