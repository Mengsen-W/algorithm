/*
 * @Date: 2021-06-18 08:01:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-18 08:28:55
 */

#include <cassert>
#include <cmath>
#include <iostream>
#include <string>
using namespace std;

string smallestGoodBase(string n) {
  int64_t nVal = stoll(n);
  int mMax = floor(log(nVal) / log(2));
  for (int m = mMax; m > 1; m--) {
    int k = pow(nVal, 1.0 / m);
    long mul = 1, sum = 1;
    for (int i = 0; i < m; i++) {
      mul *= k;
      sum += mul;
    }
    if (sum == nVal) {
      return to_string(k);
    }
  }
  return to_string(nVal - 1);
}

int main() {
  assert(smallestGoodBase("13") == "3");
  // assert(smallestGoodBase("4681") == "8");
  // assert(smallestGoodBase("1000000000000000000") == "999999999999999999");
  return 0;
}