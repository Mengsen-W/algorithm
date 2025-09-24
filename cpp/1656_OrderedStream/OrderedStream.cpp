#include <cassert>
#include <string>
#include <vector>

using namespace std;

class OrderedStream {
 public:
  OrderedStream(int n) {
    stream.resize(n + 1);
    ptr = 1;
  }

  vector<string> insert(int idKey, string value) {
    stream[idKey] = value;
    vector<string> res;
    while (ptr < stream.size() && !stream[ptr].empty()) {
      res.push_back(stream[ptr]);
      ++ptr;
    }
    return res;
  }

 private:
  vector<string> stream;
  int ptr;
};

int main() {
  OrderedStream o{5};
  assert(o.insert(3, "ccccc") == vector<string>{});
  assert(o.insert(1, "aaaaa") == vector<string>{"aaaaa"});
  assert((o.insert(2, "bbbbb") == vector<string>{"bbbbb", "ccccc"}));
  assert((o.insert(5, "eeeee") == vector<string>{}));
  assert((o.insert(4, "ddddd") == vector<string>{"ddddd", "eeeee"}));
}
