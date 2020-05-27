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
// DATA_HASHES
#include "file.h"

#define SHA256_SIZE 32

bool verify_hash()
{
  return (!DATA_EXEC_AFTER || sha256::compute((const uint8_t*)DATA_EXEC_AFTER, strlen(DATA_EXEC_AFTER)) == DATA_EXEC_AFTER_HASH);
}

array<unsigned char, SHA256_SIZE> data_hash_i(int index)
{
  array<unsigned char, SHA256_SIZE> output;
  auto pointer = DATA_HASHES + (index*SHA256_SIZE);

  static_assert(output.size() == SHA256_SIZE);
  memcpy(output.data(), pointer, SHA256_SIZE);

  return output;
}

bool verify_hash(int hash_i, const unsigned char* data, size_t len)
{
#ifdef DATA_HASHED
  return sha256::compute((const uint8_t*)data, len) == data_hash_i(hash_i);
#else
  return true;
#endif
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

template<typename Path>
struct DirTree
{
  const Path path;
  DirTree(const Path input): path(input)
  {
    fs::create_directory(input);
  }

  ~DirTree()
  {
    fs::remove_all(path);
  }

  const Path& operator &()
  {
    return path;
  }
};

constexpr uint64_t max_sz()
{
  uint64_t sz =0;
  for(int i=0;i<DATA_COUNT;i++)
    sz+=DATA_LENGTHS[i];

  return sz;
}

int main(int argc,char** argv)
{
  auto path = fs::temp_directory_path() / get_uuid();
  vector<string> vecargs;
  auto args = arg_strings(argc,argv, vecargs);

  const auto max_size = max_sz();
  static_assert(max_size == sizeof(DATA));

  if(!verify_hash())
    {
      cerr << "Error: Bad message hash" << endl;
      return 1;
    }
  
#ifndef SILENT
  cout << "Extracting " << DATA_COUNT <<  " files to " << path << "..." << endl;
#endif

  DirTree tree(path);
  uint64_t sz=0;
  for(int i=0;i<DATA_COUNT;i++)
    {
      auto data = get_data(i);
      auto data_len = DATA_LENGTHS[i];

      if ((sz+=data_len) > sizeof(DATA))
	{
#ifndef SILENT
	  cerr << "Failed: Invalid sizes (" << sz << " is larger than " << sizeof(DATA) << ")" << endl;
#endif
	  return 1;
	}
      
#ifndef SILENT
      cout << " <- " << DATA_NAMES[i] << " (" << data_len << ")" << flush;
#endif
      if(!verify_hash(i, data, data_len)) {
#ifndef SILENT
	cout << " FAILED: Invalid hash" << endl;
	cerr << "Aborting." << endl;
#endif
	return 1;
      } else {
	write_to_file(&tree / DATA_NAMES[i], data, data_len);
#ifndef SILENT
	cout << " OK" << endl;
#endif
      }
    }

  if(DATA_EXEC_AFTER) {
    string execstr = get_exec_str(vecargs, args, path);
#ifndef SILENT
    cout << "exec: "  << execstr << endl;
#endif
    system(execstr.c_str());
  }
  return 0;
}
