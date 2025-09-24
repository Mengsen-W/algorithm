#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    vector<string> removeSubfolders(vector<string>& folder) {
        ranges::sort(folder);
        vector<string> ans = {folder[0]};
        for (int i = 1; i < folder.size(); i++) {
            string& s = folder[i];
            string& last = ans.back();
            if (!s.starts_with(last) || s[last.size()] != '/') {
                ans.emplace_back(s);
            }
        }
        return ans;
    }
};


int main() {
  vector<tuple<vector<string>, vector<string>>> tests{
      {{"/a", "/a/b", "/c/d", "/c/d/e", "/c/f"}, {"/a", "/c/d", "/c/f"}},
      {{"/a", "/a/b/c", "/a/b/d"}, {"/a"}},
      {{"/a/b/c", "/a/b/ca", "/a/b/d"}, {"/a/b/c", "/a/b/ca", "/a/b/d"}},
  };

  for (auto &[folder, ans]: tests) {
    assert(Solution().removeSubfolders(folder) == ans);
  }
}