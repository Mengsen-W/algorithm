/*
 * @Date: 2021-10-19 08:44:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-19 08:48:29
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

struct TrieNode {
  vector<TrieNode *> child;
  bool isEnd;
  TrieNode() {
    this->child = vector<TrieNode *>(26, nullptr);
    this->isEnd = false;
  }
};

void insert(TrieNode *root, const string &word) {
  TrieNode *node = root;
  for (auto c : word) {
    if (node->child[c - 'a'] == nullptr) {
      node->child[c - 'a'] = new TrieNode();
    }
    node = node->child[c - 'a'];
  }
  node->isEnd = true;
}

class WordDictionary {
 public:
  WordDictionary() { trie = new TrieNode(); }

  void addWord(string word) { insert(trie, word); }

  bool search(string word) { return dfs(word, 0, trie); }

  bool dfs(const string &word, int index, TrieNode *node) {
    if (index == (int)word.size()) return node->isEnd;

    char ch = word[index];
    if (ch >= 'a' && ch <= 'z') {
      TrieNode *child = node->child[ch - 'a'];
      if (child != nullptr && dfs(word, index + 1, child)) return true;

    } else if (ch == '.') {
      for (int i = 0; i < 26; i++) {
        TrieNode *child = node->child[i];
        if (child != nullptr && dfs(word, index + 1, child)) return true;
      }
    }
    return false;
  }

 private:
  TrieNode *trie;
};

int main() {
  WordDictionary wordDictionary =  WordDictionary();
  wordDictionary.addWord("bad");
  wordDictionary.addWord("dad");
  wordDictionary.addWord("mad");
  assert(!wordDictionary.search("pad"));  // return False
  assert(wordDictionary.search("bad"));   // return True
  assert(wordDictionary.search(".ad"));   // return True
  assert(wordDictionary.search("b.."));   // return True
}