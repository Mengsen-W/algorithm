/*
 * @Date: 2023-07-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-06
 * @FilePath: /algorithm/cpp/2718_maximum_even_split/maximum_even_split.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<long long> maximumEvenSplit(long long finalSum) {
    vector<long long> res;
    if (finalSum % 2 > 0) {
      return res;
    }
    for (int i = 2; i <= finalSum; i += 2) {
      res.push_back(i);
      finalSum -= i;
    }
    res.back() += finalSum;
    return res;
  }
};

int main() {
  vector<tuple<long long, vector<long long>>> testsMap{
      {12, vector<long long>{2, 4, 6}},
      {7, vector<long long>{}},
      {28, vector<long long>{2, 4, 6, 16}},
  };

  for (auto &[finalSum, expected] : testsMap) {
    assert(Solution().maximumEvenSplit(finalSum) == expected);
  }
}
