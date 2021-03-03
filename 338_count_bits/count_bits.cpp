/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-03 09:33:03
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-03 09:42:29
 */

#include <cassert>
#include <vector>

using namespace std;

vector<int> count_bits(int num) {
  vector<int> res(num + 1, 0);
  for (int i = 1; i <= num; ++i) res[i] = res[i & (i - 1)] + 1;
  return res;
}

int main(void) {
  assert(count_bits(2) == std::move(vector<int>{0, 1, 1}));
  assert(count_bits(5) == std::move(vector<int>{0, 1, 1, 2, 1, 2}));
  return 0;
}