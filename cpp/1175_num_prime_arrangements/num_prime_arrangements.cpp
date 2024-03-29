/*
 * @Date: 2022-06-30
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-06-30
 * @FilePath: /algorithm/1175_num_prime_arrangements/num_prime_arrangements.cpp
 */

#include <cassert>

const int MOD = 1e9 + 7;

class Solution {
 public:
  int numPrimeArrangements(int n) {
    int numPrimes = 0;
    for (int i = 1; i <= n; i++) {
      if (isPrime(i)) {
        numPrimes++;
      }
    }
    return (int)(factorial(numPrimes) * factorial(n - numPrimes) % MOD);
  }

  bool isPrime(int n) {
    if (n == 1) {
      return false;
    }
    for (int i = 2; i * i <= n; i++) {
      if (n % i == 0) {
        return false;
      }
    }
    return true;
  }

  long factorial(int n) {
    long res = 1;
    for (int i = 1; i <= n; i++) {
      res *= i;
      res %= MOD;
    }
    return res;
  }
};

int main() {
  assert(Solution().numPrimeArrangements(5) == 12);
  assert(Solution().numPrimeArrangements(100) == 682289015);
}