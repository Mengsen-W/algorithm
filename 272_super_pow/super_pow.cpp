/*
 * @Date: 2021-12-05 07:19:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-05 07:22:59
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
  const int MOD = 1337;

  int pow(int x, int n) {
    int res = 1;
    while (n) {
      if (n % 2) res = (long)res * x % MOD;
      x = (long)x * x % MOD;
      n /= 2;
    }
    return res;
  }

 public:
  int superPow(int a, vector<int> b) {
    int ans = 1;
    for (int e : b) ans = (long)pow(ans, 10) * pow(a, e) % MOD;
    return ans;
  }
};

int main() {
  assert(Solution().superPow(2, {3}) == 8);
  assert(Solution().superPow(2, {1, 0}) == 1024);
  assert(Solution().superPow(1, {4,3,3,8,5,2}) == 1);
  assert(Solution().superPow(2147483647, {2, 0, 0}) == 1198);
}