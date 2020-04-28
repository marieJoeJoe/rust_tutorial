#include <stdio.h>
#include <stdlib.h>
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <X11/Xos.h>
#include <X11/Xatom.h>

/*GLX header*/
#include <GL/gl.h>
#include <GL/glx.h>
#include <GL/glu.h>

Display *dpy;
XWindowAttributes winattr;
Window win;



void DrawTriangle(void)
{

  static int angle;

  XGetWindowAttributes(dpy, win, &winattr);
  glViewport(0,0,winattr.width,winattr.height); 

  glClearColor(1.0f, 1.0f, 1.0f, 0.0f);//background color R G B
  glClear( GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT );
  glColor3f(1.0f, 0.0f, 1.0f);//triangle color RGB

  glPushMatrix();
  glRotatef(angle, 0.5, 0.5, 0.0);// axis X,Y,Z

  glBegin(GL_TRIANGLES);
  glVertex3f( 0.0f,  0.0f, 0.0f);
  glVertex3f( -0.5f, 0.5f, 0.0f);
  glVertex3f( 0.5f,  0.8f, 0.0f);

  glEnd();

  glPopMatrix();

  glXSwapBuffers(dpy, win);

  usleep(1000);

  angle += 1;
}


int main(int argc, char* argv[])
{
  int screen;
  Window root_win;
  XEvent event;

  unsigned int depth;
  XSetWindowAttributes attrs;

  /*GLX vars*/
  GLint att[] = {GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, None};
  XVisualInfo *visual;
  GLXContext glc;

  dpy = XOpenDisplay(NULL);

  if(dpy == NULL){
    fprintf(stderr,"Cannot open display");
    exit(1);
  }
  screen = DefaultScreen(dpy);
  depth = DefaultDepth(dpy,screen);
  root_win = RootWindow(dpy,screen);

  visual = glXChooseVisual(dpy, screen, att);

  attrs.border_pixel = BlackPixel(dpy,screen);
  attrs.background_pixel = WhitePixel(dpy,screen);
  attrs.override_redirect = True;
  //attrs.colormap = CopyFromParent;
  attrs.colormap = XCreateColormap(dpy, root_win, visual->visual,AllocNone);
  attrs.event_mask = ExposureMask | KeyPressMask;

  /*parent window*/
  win = XCreateWindow(dpy, root_win,
		  200,200,500,300,0,visual->depth,InputOutput,visual->visual,
		  CWBackPixel|CWColormap|CWBorderPixel|CWEventMask,&attrs);
  XMapWindow(dpy,win);

  glc = glXCreateContext(dpy, visual, NULL, GL_TRUE);
  glXMakeCurrent(dpy, win, glc);

  glEnable(GL_DEPTH_TEST);

  while(1)
  {
    
	  
    if(0 == XPending(dpy))
    {
      DrawTriangle();
      continue;    
    }

    XNextEvent(dpy,&event);
    if(Expose == event.type)
    {

#if 0      
      XGetWindowAttributes(dpy, win, &winattr);
      glViewport(0,0,winattr.width,winattr.height); 

      glClearColor(1.0f, 1.0f, 1.0f, 0.0f);//background color R G B
      glClear( GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT );
      glColor3f(0.0f, 0.0f, 0.0f);

      glBegin(GL_TRIANGLES);
      glVertex3f( 0.0f,  0.0f, 0.0f);
      glVertex3f( -0.5f, 0.5f, 0.0f);
      glVertex3f( 0.5f,  0.8f, 0.0f);

      glEnd();
      glXSwapBuffers(dpy, win);
#endif

    }

    if(KeyPress == event.type)
    {
      XDestroyWindow(dpy, win);
      XCloseDisplay(dpy);
      break;
    }
  
  }

  return 0;
}
