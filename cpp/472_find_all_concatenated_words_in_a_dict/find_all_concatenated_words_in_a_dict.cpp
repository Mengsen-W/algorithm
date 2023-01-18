/*
 * @Date: 2021-12-28 00:42:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-28 01:04:25
 */

#include <cassert>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

struct Trie {
  bool isEnd;
  vector<Trie *> children;
  Trie() {
    this->children = vector<Trie *>(26, nullptr);
    this->isEnd = false;
  }
};

class Solution {
 public:
  Trie *trie = new Trie();

  vector<string> findAllConcatenatedWordsInADict(vector<string> &words) {
    vector<string> ans;
    sort(words.begin(), words.end(),
         [&](const string &a, const string &b) { return a.size() < b.size(); });
    for (size_t i = 0; i < words.size(); i++) {
      string word = words[i];
      if (word.size() == 0) {
        continue;
      }
      vector<int> visited(word.size(), 0);
      if (dfs(word, 0, visited)) {
        ans.emplace_back(word);
      } else {
        insert(word);
      }
    }
    return ans;
  }

  bool dfs(const string &word, int start, vector<int> &visited) {
    if (static_cast<int>(word.size()) == start) {
      return true;
    }
    if (visited[start]) {
      return false;
    }
    visited[start] = true;
    Trie *node = trie;
    for (size_t i = start; i < word.size(); i++) {
      char ch = word[i];
      int index = ch - 'a';
      node = node->children[index];
      if (node == nullptr) {
        return false;
      }
      if (node->isEnd) {
        if (dfs(word, i + 1, visited)) {
          return true;
        }
      }
    }
    return false;
  }

  void insert(const string &word) {
    Trie *node = trie;
    for (size_t i = 0; i < word.size(); i++) {
      char ch = word[i];
      int index = ch - 'a';
      if (node->children[index] == nullptr) {
        node->children[index] = new Trie();
      }
      node = node->children[index];
    }
    node->isEnd = true;
  }
};

int main() {
  {
    vector<string> words{"cat", "cats",        "catsdogcats",
                         "dog", "dogcatsdog",  "hippopotamuses",
                         "rat", "ratcatdogcat"};
    vector<string> ret = Solution().findAllConcatenatedWordsInADict(words);
    sort(ret.begin(), ret.end());
    vector<string> ans{"catsdogcats", "dogcatsdog", "ratcatdogcat"};
    assert(ret == ans);
  }

  {
    vector<string> words {"cat","dog","catdog"};
    vector<string> ans {"catdog"};
    assert(Solution().findAllConcatenatedWordsInADict(words) == ans);
  }
}