/*
 * @Date: 2023-11-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-23
 * @FilePath: /algorithm/cpp/1410_entity_parser/entity_parser.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  using EntityChar = pair<string, char>;

  vector<EntityChar> entityList;

  string entityParser(string text) {
    entityList = vector({(EntityChar){"&quot;", '"'}, (EntityChar){"&apos;", '\''}, (EntityChar){"&amp;", '&'},
                         (EntityChar){"&gt;", '>'}, (EntityChar){"&lt;", '<'}, (EntityChar){"&frasl;", '/'}});

    string r = "";
    for (int pos = 0; pos < text.size();) {
      bool isEntity = false;
      if (text[pos] == '&') {
        for (const auto &[e, c] : entityList) {
          if (text.substr(pos, e.size()) == e) {
            r.push_back(c);
            pos += e.size();
            isEntity = true;
            break;
          }
        }
      }
      if (!isEntity) {
        r.push_back(text[pos++]);
        continue;
      }
    }
    return r;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"&amp; is an HTML entity but &ambassador; is not.", "& is an HTML entity but &ambassador; is not."},
      {"and I quote: &quot;...&quot;", "and I quote: \"...\""},
      {"Stay home! Practice on Leetcode :)", "Stay home! Practice on Leetcode :)"},
      {"x &gt; y &amp;&amp; x &lt; y is always false", "x > y && x < y is always false"},
      {"leetcode.com&frasl;problemset&frasl;all", "leetcode.com/problemset/all"},
  };

  for (auto &[text, ans] : tests) {
    assert(Solution().entityParser(text) == ans);
  }
}