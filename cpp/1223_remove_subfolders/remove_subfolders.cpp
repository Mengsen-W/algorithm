/*
 * @Date: 2023-02-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-08
 * @FilePath: /algorithm/cpp/1223_remove_subfolders/remove_subfolders.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

struct Trie {
  Trie() : ref(-1) {}

  unordered_map<string, Trie*> children;
  int ref;
};

class Solution {
 public:
  vector<string> removeSubfolders(vector<string>& folder) {
    auto split = [](const string& s) -> vector<string> {
      vector<string> ret;
      string cur;
      for (char ch : s) {
        if (ch == '/') {
          ret.push_back(move(cur));
          cur.clear();
        } else {
          cur.push_back(ch);
        }
      }
      ret.push_back(move(cur));
      return ret;
    };

    Trie* root = new Trie();
    for (int i = 0; i < folder.size(); ++i) {
      vector<string> path = split(folder[i]);
      Trie* cur = root;
      for (const string& name : path) {
        if (!cur->children.count(name)) {
          cur->children[name] = new Trie();
        }
        cur = cur->children[name];
      }
      cur->ref = i;
    }

    vector<string> ans;

    function<void(Trie*)> dfs = [&](Trie* cur) {
      if (cur->ref != -1) {
        ans.push_back(folder[cur->ref]);
        return;
      }
      for (auto&& [_, child] : cur->children) {
        dfs(child);
      }
    };

    dfs(root);
    return ans;
  }
};

int main() {
  {
    vector<string> folder{"/a", "/a/b", "/c/d", "/c/d/e", "/c/f"};
    vector<string> ans{"/a", "/c/d", "/c/f"};
    assert(Solution().removeSubfolders(folder) == ans);
  }

  {
    vector<string> folder{"/a", "/a/b/c", "/a/b/d"};
    vector<string> ans{"/a"};
    assert(Solution().removeSubfolders(folder) == ans);
  }

  {
    vector<string> folder{"/a/b/c", "/a/b/ca", "/a/b/d"};
    vector<string> ans{"/a/b/c", "/a/b/ca", "/a/b/d"};
    assert(Solution().removeSubfolders(folder) == ans);
  }
}