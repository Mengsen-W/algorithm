#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numEquivDominoPairs(vector<vector<int>>& dominoes) {
    vector<int> num(100);
    int ret = 0;
    for (auto& it : dominoes) {
      int val = it[0] < it[1] ? it[0] * 10 + it[1] : it[1] * 10 + it[0];
      ret += num[val];
      num[val]++;
    }
    return ret;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 2}, {2, 1}, {3, 4}, {5, 6}}, 1},
      {{{1, 2}, {1, 2}, {1, 1}, {1, 2}, {2, 2}}, 3},
  };

  for (auto &[dominoes, ans] : tests) {
    assert(Solution().numEquivDominoPairs(dominoes) == ans);
  }
}