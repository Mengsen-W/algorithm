/*
 * @Date: 2023-02-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-20
 * @FilePath: /algorithm/cpp/2347_best_hand/best_hand.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  string bestHand(vector<int>& ranks, vector<char>& suits) {
    unordered_set<char> suitsSet;
    for (char suit : suits) {
      suitsSet.emplace(suit);
    }
    if (suitsSet.size() == 1) {
      return "Flush";
    }
    unordered_map<int, int> h;
    for (int rank : ranks) {
      h[rank]++;
    }
    if (h.size() == 5) {
      return "High Card";
    }
    for (auto [_, val] : h) {
      if (val > 2) {
        return "Three of a Kind";
      }
    }
    return "Pair";
  }
};

int main() {
  {
    vector<int> ranks{13, 2, 3, 1, 9};
    vector<char> suits{'a', 'a', 'a', 'a', 'a'};
    string ans{"Flush"};
    assert(Solution().bestHand(ranks, suits) == ans);
  }

  {
    vector<int> ranks{4, 4, 2, 4, 4};
    vector<char> suits{'d', 'a', 'a', 'b', 'c'};
    string ans{"Three of a Kind"};
    assert(Solution().bestHand(ranks, suits) == ans);
  }

  {
    vector<int> ranks{10,10,2,12,9};
    vector<char> suits{'a','b','c','a','d'};
    string ans{"Pair"};
    assert(Solution().bestHand(ranks, suits) == ans);
  }
}