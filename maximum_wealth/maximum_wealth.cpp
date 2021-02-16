/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-16 21:53:30
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-16 22:08:17
 */

#include <cassert>
#include <climits>
#include <iostream>
#include <vector>

using namespace std;

int maximum_wealth(vector<vector<int>>& accounts) {
  int res = INT_MIN;
  int count = 0;
  for (vector<int>& account : accounts) {
    for (int num : account) {
      count += num;
    }
    res = max(res, count);
    count = 0;
  }

  return res;
}

int main(void) {
  vector<vector<int>> account;
  account = {{1, 2, 3}, {3, 2, 1}};
  assert(6 == maximum_wealth(account));
  account = {{1, 5}, {7, 3}, {3, 5}};
  assert(10 == maximum_wealth(account));
  account = {{2, 8, 7}, {7, 1, 3}, {1, 9, 5}};
  assert(17 == maximum_wealth(account));

  return 0;
}