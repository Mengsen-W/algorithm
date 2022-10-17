/*
 * @Date: 2022-10-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-17
 * @FilePath: /algorithm/904_total_fruit/total_fruit.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int totalFruit(vector<int>& fruits) {
    int n = fruits.size();
    unordered_map<int, int> cnt;

    int left = 0, ans = 0;
    for (int right = 0; right < n; ++right) {
      ++cnt[fruits[right]];
      while (cnt.size() > 2) {
        auto it = cnt.find(fruits[left]);
        --it->second;
        if (it->second == 0) {
          cnt.erase(it);
        }
        ++left;
      }
      ans = max(ans, right - left + 1);
    }
    return ans;
  }
};

int main() {
  {
    vector<int> fruits{1, 2, 1};
    int ans = 3;
    assert(Solution().totalFruit(fruits) == ans);
  }

  {
    vector<int> fruits{0, 1, 2, 2};
    int ans = 3;
    assert(Solution().totalFruit(fruits) == ans);
  }

  {
    vector<int> fruits{1, 2, 3, 2, 2};
    int ans = 4;
    assert(Solution().totalFruit(fruits) == ans);
  }

  {
    vector<int> fruits{3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4};
    int ans = 5;
    assert(Solution().totalFruit(fruits) == ans);
  }
}