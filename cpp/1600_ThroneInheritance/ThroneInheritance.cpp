/*
 * @Date: 2024-04-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-07
 * @FilePath: /algorithm/cpp/1600_ThroneInheritance/ThroneInheritance.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class ThroneInheritance {
 private:
  unordered_map<string, vector<string>> edges;
  unordered_set<string> dead;
  string king;

 public:
  ThroneInheritance(string kingName) : king{std::move(kingName)} {}

  void birth(string parentName, string childName) { edges[std::move(parentName)].push_back(std::move(childName)); }

  void death(string name) { dead.insert(std::move(name)); }

  vector<string> getInheritanceOrder() {
    vector<string> ans;

    function<void(const string&)> preorder = [&](const string& name) {
      if (!dead.count(name)) {
        ans.push_back(name);
      }
      if (edges.count(name)) {
        for (const string& childName : edges[name]) {
          preorder(childName);
        }
      }
    };

    preorder(king);
    return ans;
  }
};

int main() {
  ThroneInheritance t = ThroneInheritance("king");  // 继承顺序：king
  t.birth("king", "andy");                          // 继承顺序：king > andy
  t.birth("king", "bob");                           // 继承顺序：king > andy > bob
  t.birth("king", "catherine");                     // 继承顺序：king > andy > bob > catherine
  t.birth("andy", "matthew");                       // 继承顺序：king > andy > matthew > bob > catherine
  t.birth("bob", "alex");                           // 继承顺序：king > andy > matthew > bob > alex > catherine
  t.birth("bob", "asha");  // 继承顺序：king > andy > matthew > bob > alex > asha > catherine
  assert((t.getInheritanceOrder() == vector<string>{"king", "andy", "matthew", "bob", "alex", "asha", "catherine"}));
  t.death("bob");  // 继承顺序：king > andy > matthew > bob（已经去世）> alex > asha > catherine
  assert((t.getInheritanceOrder() == vector<string>{"king", "andy", "matthew", "alex", "asha", "catherine"}));
}