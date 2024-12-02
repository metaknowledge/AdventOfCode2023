#include <iostream>
#include <string_view>
#include <vector>
#include <fstream>
#include <algorithm>
#include <array>
#include <functional>
#include <iostream>
#include <string_view>
#include <sstream>


using namespace std;

int check_goodness(vector<int> level) {
  if (level.size() <= 1) {
    return 1;
  }
  if (level.at(0) - level.at(1) > 0) {
    for (int i = 0; i < level.size() - 1; i++) {
      if (
        level.at(i) - level.at(i+1) < 0 ||
        abs(level.at(i) - level.at(i+1)) > 3 ||
        level.at(i) == level.at(i+1)
      ) {
        return 0;
      }
    }
  } else {
    for (int i = 0; i < level.size() - 1; i++) {
      if (
        level.at(i) - level.at(i+1) > 0 ||
        abs(level.at(i) - level.at(i+1)) > 3 ||
        level.at(i) == level.at(i+1)
      ) {
        return 0;
      }
    }
  }
  return 1;
}


int main()
{
  ifstream f("input");
  
  string line;

  int total = 0;
  while (getline(f, line)) {
    stringstream ss;
    ss << line;
    string t;
    vector<int> level;
    while (getline(ss, t, ' ')) {
      level.push_back(stoi(t));
    }
    total += check_goodness(level);
  }


  cout << total;
  return 0;
}


