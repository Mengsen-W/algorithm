/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-16 22:42:55
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-16 22:58:23
 */

#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

// 若a^b=c,则a=b^c
vector<int> decode(vector<int>& encoded, int first) {
  int m = encoded.size();
  m++;
  vector<int> ans(m);
  ans[0] = first;
  for (auto i = 0; i < encoded.size(); ++i) {
    ans[i + 1] = ans[i] ^ encoded[i];
  }

  return ans;
}

int main() {
  vector<int> encoded{};
  int first;
  encoded = {1, 2, 3};
  first = 1;
  assert((vector<int>{1, 0, 2, 1} == decode(encoded, first)));
  encoded = {6, 2, 7, 3};
  first = 4;
  assert((vector<int>{4, 2, 0, 7, 4} == decode(encoded, first)));
}