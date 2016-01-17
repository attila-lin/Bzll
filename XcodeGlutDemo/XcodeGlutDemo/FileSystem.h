#ifndef FILESYSTEM_H_
#define FILESYSTEM_H_
#include "Stream.h"
#include <string>

namespace bzll
{
  class FileSystem
  {
  public:
    enum StreamMode
    {
        READ = 1,
        WRITE = 2
    };

    ~FileSystem();
    static bool fileExists(const std::string& filePath);
    static Stream* open(const std::string& path, size_t streamMode = READ);
  private:
    FileSystem();
  };
}
#endif
