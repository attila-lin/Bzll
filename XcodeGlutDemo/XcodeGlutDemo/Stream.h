#ifndef Stream_H_
#define Stream_H_
#include <string>
#include <stddef.h>
namespace bzll
{
  class Stream
  {
  public:
    virtual ~Stream() {};
    virtual bool canRead() = 0;
    virtual bool canWrite() = 0;
    virtual bool canSeek() = 0;
    virtual void close() = 0;
    virtual size_t read(void* ptr, size_t size, size_t count) = 0;
    virtual char* readLine(char* str, int num) = 0;
    virtual size_t write(const void* ptr, size_t size, size_t count) = 0;
  protected:
    Stream() {};
  private:
    Stream(const Stream&);            // Hidden copy constructor.
    Stream& operator=(const Stream&); // Hidden copy assignment operator.

  };
}

#endif
