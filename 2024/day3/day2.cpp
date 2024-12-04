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
#include <regex>

using namespace std;

int main()
{
  ifstream f("input");
  
  string line;
  stringstream ss;
  int total = 0;
  while (getline(f, line)) {
    ss << line;
  }
  string full = ss.str();
  regex regexp("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)");
  smatch m;
  regex dont("(don't\\(\\).*?do\\(\\))");
  full = regex_replace(full, dont, "");

  while (regex_search(full, m, regexp)) {
    cout << m[1] << m[2] << " ";
    total += stoi(m[1]) * stoi(m[2]); 
    full = m.suffix();
  }
  cout << "\n" << total << "\n";
  return 0;
}


