/*
 * @Date: 2021-10-01 09:39:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-01 10:07:12
 */

#include <cassert>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  string destCity(vector<vector<string>> &paths) {
    unordered_set<string> citiesA;
    for (auto &path : paths) citiesA.insert(path[0]);

    for (auto &path : paths)
      if (!citiesA.count(path[1])) return path[1];

    return "";
  }
};

int main() {
  vector<tuple<vector<vector<string>>, string>> tests{
      {{{"London", "New York"}, {"New York", "Lima"}, {"Lima", "Sao Paulo"}}, "Sao Paulo"},
      {{{"B", "C"}, {"D", "B"}, {"C", "A"}}, "A"},
  };

  for (auto &[paths, ans] : tests) {
    assert(Solution().destCity(paths) == ans);
  }
}
