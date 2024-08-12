/*
 * @Date: 2022-07-11
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-11
 * @FilePath: /algorithm/676_magic_dictionary/magic_dictionary.cpp
 */

#include <algorithm>
#include <cassert>
#include <functional>
#include <string>
#include <vector>

using namespace std;

struct Trie {
  bool is_finished;
  Trie* child[26];

  Trie() {
    is_finished = false;
    fill(begin(child), end(child), nullptr);
  }
};

class MagicDictionary {
 public:
  MagicDictionary() { root = new Trie(); }

  void buildDict(vector<string> dictionary) {
    for (auto&& word : dictionary) {
      Trie* cur = root;
      for (char ch : word) {
        int idx = ch - 'a';
        if (!cur->child[idx]) {
          cur->child[idx] = new Trie();
        }
        cur = cur->child[idx];
      }
      cur->is_finished = true;
    }
  }

  bool search(string searchWord) {
    function<bool(Trie*, int, bool)> dfs = [&](Trie* node, int pos, bool modified) {
      if (pos == searchWord.size()) {
        return modified && node->is_finished;
      }
      int idx = searchWord[pos] - 'a';
      if (node->child[idx]) {
        if (dfs(node->child[idx], pos + 1, modified)) {
          return true;
        }
      }
      if (!modified) {
        for (int i = 0; i < 26; ++i) {
          if (i != idx && node->child[i]) {
            if (dfs(node->child[i], pos + 1, true)) {
              return true;
            }
          }
        }
      }
      return false;
    };

    return dfs(root, 0, false);
  }

 private:
  Trie* root;
};

int main() {
  MagicDictionary m{};
  m.buildDict(vector<string>{{"hello", "leetcode"}});
  assert(m.search("hello") == false);
  assert(m.search("hhllo") == true);
  assert(m.search("hell") == false);
  assert(m.search("leetcoded") == false);
}