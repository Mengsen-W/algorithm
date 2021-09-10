/*
 * @Date: 2021-09-10 09:10:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-10 09:17:18
 */

#include <cassert>
#include <functional>
#include <vector>

using namespace std;

class Solution {
 public:
  int chalkReplacer(vector<int>& chalk, int k) {
    int n = chalk.size();
    if (chalk[0] > k) {
      return 0;
    }
    for (int i = 1; i < n; ++i) {
      chalk[i] += chalk[i - 1];
      if (chalk[i] > k) {
        return i;
      }
    }

    k %= chalk.back();
    return upper_bound(chalk.begin(), chalk.end(), k) - chalk.begin();
  }
};

int main() {
  {
    vector<int> chalk{5, 1, 5};
    int k = 22;
    int ans = 0;
    assert(Solution().chalkReplacer(chalk, k) == ans);
  }
  {
    vector<int> chalk{3, 4, 1, 2};
    int k = 25;
    int ans = 1;
    assert(Solution().chalkReplacer(chalk, k) == ans);
  }
}