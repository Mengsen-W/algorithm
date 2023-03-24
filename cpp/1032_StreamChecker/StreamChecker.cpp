/*
 * @Date: 2023-03-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-24
 * @FilePath: /algorithm/cpp/1032_StreamChecker/StreamChecker.cpp
 */

#include <cassert>
#include <queue>
#include <string>
#include <vector>

using namespace std;

typedef struct TrieNode {
  vector<TrieNode *> children;
  bool isEnd;
  TrieNode *fail;
  TrieNode() {
    this->children = vector<TrieNode *>(26, nullptr);
    this->isEnd = false;
    this->fail = nullptr;
  }
};

class StreamChecker {
 public:
  TrieNode *root;
  TrieNode *temp;
  StreamChecker(vector<string> &words) {
    root = new TrieNode();
    for (string &word : words) {
      TrieNode *cur = root;
      for (int i = 0; i < word.size(); i++) {
        int index = word[i] - 'a';
        if (cur->children[index] == nullptr) {
          cur->children[index] = new TrieNode();
        }
        cur = cur->children[index];
      }
      cur->isEnd = true;
    }
    root->fail = root;
    queue<TrieNode *> q;
    for (int i = 0; i < 26; i++) {
      if (root->children[i] != nullptr) {
        root->children[i]->fail = root;
        q.emplace(root->children[i]);
      } else {
        root->children[i] = root;
      }
    }
    while (!q.empty()) {
      TrieNode *node = q.front();
      q.pop();
      node->isEnd = node->isEnd || node->fail->isEnd;
      for (int i = 0; i < 26; i++) {
        if (node->children[i] != nullptr) {
          node->children[i]->fail = node->fail->children[i];
          q.emplace(node->children[i]);
        } else {
          node->children[i] = node->fail->children[i];
        }
      }
    }

    temp = root;
  }

  bool query(char letter) {
    temp = temp->children[letter - 'a'];
    return temp->isEnd;
  }
};

int main() {
  vector<string> words{"cd", "f", "kl"};
  StreamChecker s = StreamChecker{words};
  assert(s.query('a') == false);
  assert(s.query('b') == false);
  assert(s.query('c') == false);
  assert(s.query('d') == true);
  assert(s.query('e') == false);
  assert(s.query('f') == true);
  assert(s.query('h') == false);
  assert(s.query('i') == false);
  assert(s.query('j') == false);
  assert(s.query('k') == false);
  assert(s.query('l') == true);
}