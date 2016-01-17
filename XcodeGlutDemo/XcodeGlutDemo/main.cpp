//
//  main.cpp
//  XcodeGlutDemo
//
//  Created by whatever on 17/01/2016.
//  Copyright © 2016 whatever. All rights reserved.
//
#include "FileSystem.h"

#include <iostream>

#include <GLFW/glfw3.h>
#include <gperftools/tcmalloc.h>
#include <stdlib.h>
#include <stdio.h>


static void error_callback(int error, const char* description)
{
    fputs(description, stderr);
}
static void key_callback(GLFWwindow* window, int key, int scancode, int action, int mods)
{
    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS)
        glfwSetWindowShouldClose(window, GL_TRUE);
}

// Create a NULL-terminated string by reading the provided file
static char*
readShaderSource(const char* shaderFile)
{
    FILE* fp = fopen(shaderFile, "r");

    if ( fp == NULL ) { return NULL; }

    fseek(fp, 0L, SEEK_END);
    long size = ftell(fp);

    fseek(fp, 0L, SEEK_SET);
    char* buf = new char[size + 1];
    fread(buf, 1, size, fp);

    buf[size] = '\0';
    fclose(fp);

    return buf;
}


// Create a GLSL program object from vertex and fragment shader files
GLuint
InitShader(const char* vShaderFile, const char* fShaderFile)
{
    struct Shader {
        const char*  filename;
        GLenum       type;
        GLchar*      source;
    }  shaders[2] = {
        { vShaderFile, GL_VERTEX_SHADER, NULL },
        { fShaderFile, GL_FRAGMENT_SHADER, NULL }
    };

    GLuint program = glCreateProgram();

    for ( int i = 0; i < 2; ++i ) {
        Shader& s = shaders[i];
        s.source = readShaderSource( s.filename );
        if ( shaders[i].source == NULL ) {
            std::cerr << "Failed to read " << s.filename << std::endl;
            exit( EXIT_FAILURE );
        }

        GLuint shader = glCreateShader( s.type );
        glShaderSource( shader, 1, (const GLchar**) &s.source, NULL );
        glCompileShader( shader );

        GLint  compiled;
        glGetShaderiv( shader, GL_COMPILE_STATUS, &compiled );
        if ( !compiled ) {
            std::cerr << s.filename << " failed to compile:" << std::endl;
            GLint  logSize;
            glGetShaderiv( shader, GL_INFO_LOG_LENGTH, &logSize );
            char* logMsg = new char[logSize];
            glGetShaderInfoLog( shader, logSize, NULL, logMsg );
            std::cerr << logMsg << std::endl;
            delete [] logMsg;

            exit( EXIT_FAILURE );
        }

        delete [] s.source;

        glAttachShader( program, shader );
    }

    /* link  and error check */
    glLinkProgram(program);

    GLint  linked;
    glGetProgramiv( program, GL_LINK_STATUS, &linked );
    if ( !linked ) {
        std::cerr << "Shader program failed to link" << std::endl;
        GLint  logSize;
        glGetProgramiv( program, GL_INFO_LOG_LENGTH, &logSize);
        char* logMsg = new char[logSize];
        glGetProgramInfoLog( program, logSize, NULL, logMsg );
        std::cerr << logMsg << std::endl;
        delete [] logMsg;

        exit( EXIT_FAILURE );
    }

    /* use program object */
    glUseProgram(program);

    return program;
}


typedef struct
{
    // Handle to a program object
    GLuint programObject;
} UserData;

GLuint LoadShader ( GLenum type, const char *shaderSrc )
{
    GLuint shader;
    GLint compiled;

    // Create the shader object
    shader = glCreateShader ( type );
    if ( shader == 0 )
        return 0;
    // Load the shader source
    glShaderSource ( shader, 1, &shaderSrc, NULL );
    // Compile the shader
    // TODO 预编译
    glCompileShader ( shader );
    // Check the compile status
    glGetShaderiv ( shader, GL_COMPILE_STATUS, &compiled );
    if ( !compiled )
    {
        GLint infoLen = 0;
        glGetShaderiv ( shader, GL_INFO_LOG_LENGTH, &infoLen );
        if ( infoLen > 1 )
        {
            char* infoLog = (char *)valloc (sizeof(char) * infoLen );
            glGetShaderInfoLog( shader, infoLen, NULL, infoLog );
            printf ( "Error compiling shader:\n%s\n", infoLog );
            free ( infoLog );
        }
        glDeleteShader ( shader );
        return 0;
    }
    return shader;
}

void display(GLFWwindow* window)
{
  float ratio;
  int width, height;
  glfwGetFramebufferSize(window, &width, &height);
  ratio = width / (float) height;
  glViewport(0, 0, width, height);
  glClear(GL_COLOR_BUFFER_BIT);
  glMatrixMode(GL_PROJECTION);
  glLoadIdentity();
  glOrtho(-ratio, ratio, -1.f, 1.f, 1.f, -1.f);
  glMatrixMode(GL_MODELVIEW);
  glLoadIdentity();
  glRotatef((float) glfwGetTime() * 50.f, 0.f, 0.f, 1.f);
  glBegin(GL_TRIANGLES);
  glColor3f(1.f, 0.f, 0.f);
  glVertex3f(-0.6f, -0.4f, 0.f);
  glColor3f(0.f, 1.f, 0.f);
  glVertex3f(0.6f, -0.4f, 0.f);
  glColor3f(0.f, 0.f, 1.f);
  glVertex3f(0.f, 0.6f, 0.f);
  glEnd();
}

int main(void)
{
    GLFWwindow* window;
    glfwSetErrorCallback(error_callback);
    if (!glfwInit())
        exit(EXIT_FAILURE);

    window = glfwCreateWindow(1280, 720, "Simple example", NULL, NULL);
    if (!window)
    {
        glfwTerminate();
        exit(EXIT_FAILURE);
    }
    glfwMakeContextCurrent(window);
    glfwSwapInterval(1);
    glfwSetKeyCallback(window, key_callback);

    // Dark blue background
	  glClearColor(0.0f, 0.0f, 0.4f, 0.0f);

    // 添加鼠标
    // GLFWcursor* cursor = glfwCreateCursor(")
    // 版本信息
    std::cout << glfwGetVersionString() << std::endl;

    // test FileSystem
    std::string path = "/Users/whatever/projects/bzll/README.md";
    std::cout << path << std::endl;
    bool isExist = bzll::FileSystem::fileExists(path);
    std::cout << std::boolalpha <<isExist << std::endl;

    // FPS等调试信息
    // http://www.opengl-tutorial.org/miscellaneous/an-fps-counter/
    double lastTime = glfwGetTime();
    int nbFrames = 0;

    GLuint programID = InitShader( "/Users/whatever/projects/bzll/XcodeGlutDemo/Shader/SimpleVertexShader.vertexshader", "/Users/whatever/projects/bzll/XcodeGlutDemo/Shader/SimpleFragmentShader.fragmentshader" );
    while (!glfwWindowShouldClose(window))
    {
        double currentTime = glfwGetTime();
        nbFrames++;
        if ( currentTime - lastTime >= 1.0 ){
            printf("%f ms/frame\n", 1000.0/double(nbFrames));
            nbFrames = 0;
            lastTime += 1.0;
        }

        /* Render here */
        // Clear the screen
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        glUseProgram(programID);
        display(window);

        /* Swap front and back buffers */
        glfwSwapBuffers(window);
        /* Poll for and process events */
        glfwPollEvents();
    }
    glfwDestroyWindow(window);
    glfwTerminate();
    exit(EXIT_SUCCESS);
}
