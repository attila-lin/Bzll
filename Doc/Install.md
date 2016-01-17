# Install
---

##1. install glfw3
### 1.
```
brew update
brew tap homebrew/versions
brew install glfw3
```

### 2. xcode
+ 1. Create a new Xcode project
+ 2. OS X -> Application -> Command Line Tool
+ 3. 设置名字之类的
+ 4. open [link](http://www.glfw.org/docs/latest/quick.html#quick_init_term), and copy the code to main.cpp
+ 5. In "Build Phases" -> Link Binary With Library, add library "OpenGL.framework", and add other, in "/usr/local/Cellar/glfw3/3.1.2" has a "libglfw3.X.X.dylib"
```
brew info glfw3 # get the install position
```
+ 6. In "Build Settings" -> "Search Paths" -> "Header Search Paths", add glfw3 headers.
+ 7. build success


##2. gperftools/tcmalloc
```
brew install gperftools
```
