/*
 * @Date: 2023-05-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-19
 * @FilePath: /algorithm/cpp/1079_num_tile_possibilities/num_tile_possibilities.cpp
 */

#include <cassert>
#include <set>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
 public:
  int numTilePossibilities(string tiles) {
    unordered_map<char, int> count;
    set<char> tile;
    int n = tiles.length();
    for (char c : tiles) {
      count[c]++;
      tile.insert(c);
    }
    return dfs(count, tile, n) - 1;
  }

  int dfs(unordered_map<char, int>& count, set<char>& tile, int i) {
    if (i == 0) {
      return 1;
    }
    int res = 1;
    for (char t : tile) {
      if (count[t] > 0) {
        count[t]--;
        res += dfs(count, tile, i - 1);
        count[t]++;
      }
    }
    return res;
  }
};

int main() {
  {
    string tiles = "AAB";
    int ans = 8;
    assert(Solution().numTilePossibilities(tiles) == ans);
  }

  {
    string tiles = "AAABBC";
    int ans = 188;
    assert(Solution().numTilePossibilities(tiles) == ans);
  }

  {
    string tiles = "V";
    int ans = 1;
    assert(Solution().numTilePossibilities(tiles) == ans);
  }
}