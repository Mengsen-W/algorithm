/*
 * @Date: 2023-05-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-06
 * @FilePath: /algorithm/cpp/1419_min_number_of_frogs/min_number_of_frogs.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int minNumberOfFrogs(string croakOfFrogs) {
    if (croakOfFrogs.size() % 5 != 0) {
      return -1;
    }
    int res = 0, frogNum = 0;
    vector<int> cnt(4);
    unordered_map<char, int> mp = {{'c', 0}, {'r', 1}, {'o', 2}, {'a', 3}, {'k', 4}};
    for (char c : croakOfFrogs) {
      int t = mp[c];
      if (t == 0) {
        cnt[t]++;
        frogNum++;
        if (frogNum > res) {
          res = frogNum;
        }
      } else {
        if (cnt[t - 1] == 0) {
          return -1;
        }
        cnt[t - 1]--;
        if (t == 4) {
          frogNum--;
        } else {
          cnt[t]++;
        }
      }
    }
    if (frogNum > 0) {
      return -1;
    }
    return res;
  }
};

int main() {
  {
    string croakOfFrogs = "croakcroak";
    int ans = 1;
    assert(Solution().minNumberOfFrogs(croakOfFrogs) == ans);
  }

  {
    string croakOfFrogs = "crcoakroak";
    int ans = 2;
    assert(Solution().minNumberOfFrogs(croakOfFrogs) == ans);
  }

  {
    string croakOfFrogs = "croakcrook";
    int ans = -1;
    assert(Solution().minNumberOfFrogs(croakOfFrogs) == ans);
  }
}