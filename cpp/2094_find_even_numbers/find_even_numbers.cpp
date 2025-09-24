#include <algorithm>
#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findEvenNumbers(vector<int>& digits) {
    vector<int> res;               // 目标偶数数组
    unordered_map<int, int> freq;  // 整数数组中各数字的出现次数
    for (const int digit : digits) {
      ++freq[digit];
    }
    // 枚举所有三位偶数，维护整数中各数位的出现次数并比较判断是否为目标偶数
    for (int i = 100; i < 1000; i += 2) {
      unordered_map<int, int> freq1;
      int tmp = i;
      while (tmp) {
        ++freq1[tmp % 10];
        tmp /= 10;
      }
      if (all_of(freq1.begin(), freq1.end(), [&](const auto& x) { return freq[x.first] >= freq1[x.first]; })) {
        res.push_back(i);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests {
    {{2,1,3,0}, {102,120,130,132,210,230,302,310,312,320}},
    {{2,2,8,8,2}, {222,228,282,288,822,828,882}},
    {{3,7,5}, {}},
  };

  for (auto &[digits, ans] : tests) {
    assert(Solution().findEvenNumbers(digits) == ans);
  }
}