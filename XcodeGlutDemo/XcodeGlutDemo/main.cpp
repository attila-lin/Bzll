//
//  main.cpp
//  XcodeGlutDemo
//
//  Created by whatever on 17/01/2016.
//  Copyright © 2016 whatever. All rights reserved.
//


#include <iostream>

#include <GLFW/glfw3.h>
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
            char* infoLog = (char *)malloc (sizeof(char) * infoLen );
            glGetShaderInfoLog( shader, infoLen, NULL, infoLog );
            printf ( "Error compiling shader:\n%s\n", infoLog );
            free ( infoLog );
        }
        glDeleteShader ( shader );
        return 0;
    }
    return shader;
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
    
    // 添加鼠标
    // GLFWcursor* cursor = glfwCreateCursor(")
    std::cout << glfwGetVersionString() << std::endl;
    
    // FPS
    // http://www.opengl-tutorial.org/miscellaneous/an-fps-counter/
    double lastTime = glfwGetTime();
    int nbFrames = 0;
    
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
        /* Swap front and back buffers */
        glfwSwapBuffers(window);
        /* Poll for and process events */
        glfwPollEvents();
    }
    glfwDestroyWindow(window);
    glfwTerminate();
    exit(EXIT_SUCCESS);
}
