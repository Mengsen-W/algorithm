/*
 * @Date: 2021-05-06 10:12:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-06 10:19:27
 */

#include <cassert>
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
  {
    vector<int> encoded{1, 2, 3};
    int first = 1;
    assert(decode(encoded, first) == std::move(vector<int>{1, 0, 2, 1}));
  }
  {
    vector<int> encoded{6, 2, 7, 3};
    int first = 4;
    assert(decode(encoded, first) == std::move(vector<int>{4, 2, 0, 7, 4}));
  }

  return 0;
}