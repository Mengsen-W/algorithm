/*
 * @Date: 2021-04-14 09:36:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-14 10:31:05
 */

#include <cassert>
#include <cstring>
#include <string>
using namespace std;

class Trie {
 public:
  Trie() {
    isEnd = false;
    memset(next, 0, sizeof(next));
  }

  void insert(string word) {
    Trie* node = this;
    for (char c : word) {
      if (node->next[c - 'a'] == NULL) {
        node->next[c - 'a'] = new Trie();
      }
      node = node->next[c - 'a'];
    }
    node->isEnd = true;
  }

  bool search(string word) {
    Trie* node = this;
    for (char c : word) {
      node = node->next[c - 'a'];
      if (node == NULL) {
        return false;
      }
    }
    return node->isEnd;
  }

  bool startsWith(string prefix) {
    Trie* node = this;
    for (char c : prefix) {
      node = node->next[c - 'a'];
      if (node == NULL) {
        return false;
      }
    }
    return true;
  }

 private:
  bool isEnd;
  Trie* next[26];
};

int main() {
  Trie* obj = new Trie();
  obj->insert("apple");
  assert(obj->search("apple"));
  assert(!obj->search("app"));
  assert(obj->startsWith("app"));
  obj->insert("app");
  assert(obj->search("app"));
}