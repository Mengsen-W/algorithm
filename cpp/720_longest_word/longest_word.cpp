/*
 * @Date: 2022-03-17 14:29:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-17 14:33:09
 * @FilePath: /algorithm/720_longest_word/longest_word.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Trie {
 public:
  Trie() {
    this->children = vector<Trie *>(26, nullptr);
    this->isEnd = false;
  }

  bool insert(const string &word) {
    Trie *node = this;
    for (const auto &ch : word) {
      int index = ch - 'a';
      if (node->children[index] == nullptr) {
        node->children[index] = new Trie();
      }
      node = node->children[index];
    }
    node->isEnd = true;
    return true;
  }

  bool search(const string &word) {
    Trie *node = this;
    for (const auto &ch : word) {
      int index = ch - 'a';
      if (node->children[index] == nullptr || !node->children[index]->isEnd) {
        return false;
      }
      node = node->children[index];
    }
    return node != nullptr && node->isEnd;
  }

 private:
  vector<Trie *> children;
  bool isEnd;
};

class Solution {
 public:
  string longestWord(vector<string> words) {
    Trie trie;
    for (const auto &word : words) {
      trie.insert(word);
    }
    string longest = "";
    for (const auto &word : words) {
      if (trie.search(word)) {
        if (word.size() > longest.size() ||
            (word.size() == longest.size() && word < longest)) {
          longest = word;
        }
      }
    }
    return longest;
  }
};

int main() {
  assert(Solution().longestWord({"w", "wo", "wor", "worl", "world"}) ==
         "world");
  assert(Solution().longestWord({"a", "banana", "app", "appl", "ap", "apply",
                                 "apple"}) == "apple");
}