/*
 * @Date: 2022-02-07 05:30:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-07 05:38:14
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  string longestDiverseString(int a, int b, int c) {
    string res;
    vector<pair<int, char>> arr = {{a, 'a'}, {b, 'b'}, {c, 'c'}};

    while (true) {
      sort(arr.begin(), arr.end(),
           [](const pair<int, char>& p1, const pair<int, char>& p2) {
             return p1.first > p2.first;
           });
      bool hasNext = false;
      for (auto& [freq, ch] : arr) {
        int m = res.size();
        if (freq <= 0) {
          break;
        }
        if (m >= 2 && res[m - 2] == ch && res[m - 1] == ch) {
          continue;
        }
        hasNext = true;
        res.push_back(ch);
        freq--;
        break;
      }
      if (!hasNext) {
        break;
      }
    }

    return res;
  }
};

int main() {
  assert(Solution().longestDiverseString(1, 1, 7) == "ccaccbcc");
  assert(Solution().longestDiverseString(2, 2, 1) == "abbac");
  assert(Solution().longestDiverseString(7, 1, 0) == "aabaa");
}