/*
 * @Date: 2023-07-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-24
 * @FilePath: /algorithm/cpp/771_num_jewels_in_stones/num_jewels_in_stones.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int numJewelsInStones(string jewels, string stones) {
    int jewelsCount = 0;
    unordered_set<char> jewelsSet;
    int jewelsLength = jewels.length(), stonesLength = stones.length();
    for (int i = 0; i < jewelsLength; i++) {
      char jewel = jewels[i];
      jewelsSet.insert(jewel);
    }
    for (int i = 0; i < stonesLength; i++) {
      char stone = stones[i];
      if (jewelsSet.count(stone)) {
        jewelsCount++;
      }
    }
    return jewelsCount;
  }
};

int main() {
  vector<tuple<string, string, int>> tests{
      {"aA", "aAAbbbb", 3},
      {"z", "ZZ", 0},
  };
  for (auto &[jewels, stones, expected] : tests) {
    assert(Solution().numJewelsInStones(jewels, stones) == expected); 
  }
}