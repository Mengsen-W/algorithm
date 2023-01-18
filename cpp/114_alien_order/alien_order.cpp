/*
 * @Date: 2022-05-31 10:03:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-31 10:16:59
 * @FilePath: /algorithm/114_alien_order/alien_order.cpp
 */

#include <cassert>
#include <queue>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
 public:
  unordered_map<char, vector<char>> edges;
  unordered_map<char, int> indegrees;
  bool valid = true;

  string alienOrder(vector<string> words) {
    int length = words.size();
    for (auto word : words) {
      int wordLength = word.size();
      for (int j = 0; j < wordLength; j++) {
        char c = word[j];
        if (!edges.count(c)) {
          edges[c] = vector<char>();
        }
      }
    }
    for (int i = 1; i < length && valid; i++) {
      addEdge(words[i - 1], words[i]);
    }
    if (!valid) {
      return "";
    }
    queue<char> qu;
    for (auto [u, _] : edges) {
      if (!indegrees.count(u)) {
        qu.emplace(u);
      }
    }
    string order;
    while (!qu.empty()) {
      char u = qu.front();
      qu.pop();
      order.push_back(u);
      for (char v : edges[u]) {
        indegrees[v]--;
        if (indegrees[v] == 0) {
          qu.emplace(v);
        }
      }
    }
    return order.size() == edges.size() ? order : "";
  }

  void addEdge(string before, string after) {
    int length1 = before.size(), length2 = after.size();
    int length = min(length1, length2);
    int index = 0;
    while (index < length) {
      char c1 = before[index], c2 = after[index];
      if (c1 != c2) {
        edges[c1].emplace_back(c2);
        indegrees[c2] += 1;
        break;
      }
      index++;
    }
    if (index == length && length1 > length2) {
      valid = false;
    }
  }
};

int main() {
  assert(Solution().alienOrder({"wrt", "wrf", "er", "ett", "rftt"}) == "wertf");
  assert(Solution().alienOrder({"z", "x"}) == "zx");
  assert(Solution().alienOrder({"z", "x", "z"}) == "");
}