/*
 * @Date: 2022-10-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-29
 * @FilePath: /algorithm/1773_count_matches/count_matches.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int countMatches(vector<vector<string>>& items, string ruleKey, string ruleValue) {
    unordered_map<string, int> dictionary = {{"type", 0}, {"color", 1}, {"name", 2}};
    int res = 0, index = dictionary[ruleKey];
    for (auto&& item : items) {
      if (item[index] == ruleValue) {
        res++;
      }
    }
    return res;
  }
};

int main() {
  {
    vector<vector<string>> items{
        {"phone", "blue", "pixel"}, {"computer", "silver", "lenovo"}, {"phone", "gold", "iphone"}};
    string ruleKey = "color";
    string ruleValue = "silver";
    int ans = 1;
    assert(Solution().countMatches(items, ruleKey, ruleValue) == ans);
  }

  {
    vector<vector<string>> items{
        {"phone", "blue", "pixel"}, {"computer", "silver", "phone"}, {"phone", "gold", "iphone"}};
    string ruleKey = "type";
    string ruleValue = "phone";
    int ans = 2;
    assert(Solution().countMatches(items, ruleKey, ruleValue) == ans);
  }
}
