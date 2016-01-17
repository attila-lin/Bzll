#include "FileSystem.h"
#include "Stream.h"
#include <unistd.h>
#include <string>
#include <stddef.h>
#include <sys/types.h>
#include <sys/stat.h>

namespace bzll
{
  static std::string __resourcePath("./");
  static std::string __assetPath("");
  // static std::map<std::string, std::string> __aliases;

  class FileStream : public Stream
  {
  public:
      friend class FileSystem;

      ~FileStream();
      virtual bool canRead();
      virtual bool canWrite();
      virtual bool canSeek();
      virtual void close();
      virtual size_t read(void* ptr, size_t size, size_t count);
      virtual char* readLine(char* str, int num);
      virtual size_t write(const void* ptr, size_t size, size_t count);
      virtual bool eof();
      virtual size_t length();
      virtual long int position();
      virtual bool seek(long int offset, int origin);
      virtual bool rewind();

      static FileStream* create(const char* filePath, const char* mode);

  private:
      FileStream(FILE* file);

  private:
      FILE* _file;
      bool _canRead;
      bool _canWrite;
  };

  FileSystem::FileSystem()
  {
  }

  FileSystem::~FileSystem()
  {
  }

  // http://stackoverflow.com/questions/12774207/fastest-way-to-check-if-a-file-exist-using-standard-c-c11-c
  // fastest
  bool FileSystem::fileExists(const std::string& name)
  {
    struct stat buffer;
    return (stat (name.c_str(), &buffer) == 0);
  }

  Stream* open(const std::string& path, size_t streamMode = FileSystem::READ)
  {
    return NULL;
  }
}
