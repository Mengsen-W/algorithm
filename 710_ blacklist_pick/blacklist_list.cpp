/*
 * @Date: 2022-06-26
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-26
 * @FilePath: /algorithm/710_ blacklist_pick/blacklist_list.cpp
 */

#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
  unordered_map<int, int> b2w;
  int bound;

 public:
  Solution(int n, vector<int> &blacklist) {
    int m = blacklist.size();

    bound = n - m;
    unordered_set<int> black;
    for (int b : blacklist) {
      if (b >= bound) {
        black.emplace(b);
      }
    }

    int w = bound;
    for (int b : blacklist) {
      if (b < bound) {
        while (black.count(w)) {
          ++w;
        }
        b2w[b] = w++;
      }
    }
  }

  int pick() {
    int x = rand() % bound;
    return b2w.count(x) ? b2w[x] : x;
  }
};

int main() {}