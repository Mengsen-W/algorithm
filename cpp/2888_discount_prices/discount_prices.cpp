#include <cassert>
#include <iomanip>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string discountPrices(string sentence, int discount) {
    stringstream sin(sentence), sout;
    sout << fixed << setprecision(2);

    vector<string> words;
    string word;
    while (sin >> word) {
      if (word[0] == '$' && word.size() > 1 && all_of(word.begin() + 1, word.end(), ::isdigit)) {
        double price = stoll(word.substr(1, word.size() - 1)) * (1.0 - discount / 100.0);
        sout << '$' << price;
      } else {
        sout << word;
      }
      sout << " ";
    }
    string ans = sout.str();
    ans.pop_back();
    return ans;
  }
};

int main() {
  vector<tuple<string, int, string>> tests{
      {"there are $1 $2 and 5$ candies in the shop", 50, "there are $0.50 $1.00 and 5$ candies in the shop"},
      {"1 2 $3 4 $5 $6 7 8$ $9 $10$", 100, "1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$"},
  };

  for (auto &[sentence, discount, ans] : tests) {
    assert(Solution().discountPrices(sentence, discount) == ans);
  }
}