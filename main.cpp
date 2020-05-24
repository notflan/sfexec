#include <iostream>
#include <sstream>
#include <fstream>
#include <filesystem>
#include <string>
#include <random>
#include <vector>
#include <regex>

#pragma GCC diagnostic ignored "-Wattributes"

#include <sha256_literal.h>
#include <sha256.h>

using namespace std;
namespace fs = std::filesystem;

string get_uuid() {
  static random_device dev;
  static mt19937 rng(dev());

  uniform_int_distribution<int> dist(0, 15);

  const char *v = "0123456789abcdef";
  const bool dash[] = { 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0 };

  string res;
  for (int i = 0; i < 16; i++) {
    if (dash[i]) res += "-";
    res += v[dist(rng)];
    res += v[dist(rng)];
  }
  return res;
}

// DATA
// DATA_LENGTHS
// DATA_NAMES
// DATA_COUNT
// DATA_EXEC_AFTER
// DATA_EXEC_AFTER_HASH
#include "file.h"

bool verify_hash()
{
  
  return (!DATA_EXEC_AFTER || sha256::compute((const uint8_t*)DATA_EXEC_AFTER, strlen(DATA_EXEC_AFTER)) == DATA_EXEC_AFTER_HASH);
}

string arg_vec_int(int i)
{
  stringstream ss;
  ss << "%arg\\[" << i << "\\]";
  return ss.str();
}

string get_exec_str(vector<string> args, string args_full, string loc)
{
  string str(DATA_EXEC_AFTER);
  
  str = regex_replace(str, std::regex("%location"), loc);

  str = regex_replace(str, std::regex("%argc"), std::to_string(args.size()));
  
  str = regex_replace(str, std::regex("%args"), args_full);

  
  for(int i=0;i<args.size();i++)
    {
      str = regex_replace(str, std::regex(arg_vec_int(i)), args[i]);
    }

  
  return str;
}

string arg_strings(int argc,char** argv, vector<string>& output)
{
  stringstream ss;
  for(int i=1;i<argc;i++)
    {
      auto argi = string(argv[i]);
      ss << argi;
      if(i!=argc-1)
	ss << ' ';
      output.push_back(argi);
    }
  return ss.str();
}

void write_to_file(string to, const unsigned char* data, long length)
{
  ofstream file;
  file.open(to, ios::out | ios::binary );
  file.write((const char*)data, length);
  file.close();
}

long get_index(int i)
{
  if(i<=0) return 0;
  else return get_index(i-2) + DATA_LENGTHS[i-1];
}

const unsigned char* get_data(int i)
{
  const unsigned char* data = DATA;
  if(i==0) return DATA;
  for(int j=0;j<i;j++)
    {
      data+=DATA_LENGTHS[j];
    }
  return data;
}

int main(int argc,char** argv)
{
  auto path = fs::temp_directory_path() / get_uuid();
  vector<string> vecargs;
  auto args = arg_strings(argc,argv, vecargs);

  if(!verify_hash())
    {
      cerr << "Error: Bad message hash\n";
      return 1;
    }
  
#ifndef SILENT
  cout << "Extracting " << DATA_COUNT <<  " files to " << path << "...\n";
#endif

  fs::create_directory(path);
  for(int i=0;i<DATA_COUNT;i++)
    {
#ifndef SILENT
      cout << " <- " << DATA_NAMES[i] << " (" << DATA_LENGTHS[i] << ")\n";
#endif
      write_to_file(path / DATA_NAMES[i], get_data(i), DATA_LENGTHS[i]);
    }

  if(DATA_EXEC_AFTER) {
    string execstr = get_exec_str(vecargs, args, path);
#ifndef SILENT
    cout << "exec: "  << execstr << '\n';
#endif
    system(execstr.c_str());
  }
  fs::remove_all(path);
  return 0;
}