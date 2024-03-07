/*
 * @Date: 2024-03-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-07
 * @FilePath: /algorithm/cpp/2575_divisibility_array/divisibility_array.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> divisibilityArray(string word, int m) {
    vector<int> res;
    long long cur = 0;
    for (char& c : word) {
      cur = (cur * 10 + (c - '0')) % m;
      res.push_back(cur == 0 ? 1 : 0);
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int, vector<int>>> tests{
      {"998244353", 3, {1, 1, 0, 0, 0, 1, 1, 0, 0}},
      {"1010", 10, {0, 1, 0, 1}},
  };

  for (auto& [word, m, ans] : tests) {
    assert(Solution().divisibilityArray(word, m) == ans);
  }
}