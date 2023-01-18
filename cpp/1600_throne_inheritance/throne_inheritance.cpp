/*
 * @Date: 2021-06-20 09:48:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-20 10:25:27
 */

#include <functional>
#include <iostream>
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
  ThroneInheritance(string kingName) : king{move(kingName)} {}

  void birth(string parentName, string childName) {
    edges[move(parentName)].push_back(move(childName));
  }

  void death(string name) { dead.insert(move(name)); }

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
  ThroneInheritance* t = new ThroneInheritance("king");

  t->birth("king", "andy");       // 继承顺序：king > andy
  t->birth("king", "bob");        // 继承顺序：king > andy > bob
  t->birth("king", "catherine");  // 继承顺序：king > andy > bob > catherine
  t->birth("andy",
           "matthew");  // 继承顺序：king > andy > matthew > bob > catherine
  t->birth("bob",
           "alex");  // 继承顺序：king > andy > matthew > bob > alex > catherine
  t->birth("bob", "asha");  // 继承顺序：king > andy > matthew > bob > alex >
                            // asha > catherine
  vector<string> inheritance_order{};
  inheritance_order = t->getInheritanceOrder();
  for (string& i : inheritance_order) cout << i << ',';
  cout << endl;
  t->death("bob");
  inheritance_order = t->getInheritanceOrder();
  for (string& i : inheritance_order) cout << i << ',';
  cout << endl;

  return 0;
}
