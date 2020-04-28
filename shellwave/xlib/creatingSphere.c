#include <stdio.h>
#include <stdlib.h>
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <X11/Xos.h>
#include <X11/Xatom.h>

#include <math.h>

/*GLX header*/
#include <GL/gl.h>
#include <GL/glx.h>
#include <GL/glu.h>

Display *dpy;
XWindowAttributes winattr;
Window win;

#define RESL 0.02

void DrawTriangle(void)
{

  static int angle;
  static float pos = 0.0;

  float angle1, angle2;
  float xx, yy, zz, xxp, yyp, zzp;

  XGetWindowAttributes(dpy, win, &winattr);
  glViewport(0,0,winattr.width,winattr.height); 

  glClearColor(1.0f, 1.0f, 1.0f, 0.0f);//background color R G B
  glClear( GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT );
  //glColor3f(0.0f, 0.9f, 1.0f);//triangle color RGB

  glPushMatrix();
  glRotatef(angle, 1.0, 0.3, 0.0); 
  // axis X,Y,Z
  //glScalef(pos,pos,0.0);


  for(angle2 = -3.1316/2.0; angle2 <= 3.1416/2.0; angle2 += RESL)
  {
  
    glBegin(GL_QUAD_STRIP);
    for(angle1 = 0.0; angle1 <= 3.1416*2 + RESL; angle1 += RESL)
    {
      yy = sin(angle2);
      xx = cos(angle1)*cos(angle2);
      zz = sin(angle1)*cos(angle2);

      yyp = sin( angle2 + RESL );
      xxp = cos(angle1)*cos(angle2 + RESL);
      zzp = sin(angle1)*cos(angle2 + RESL);

      glColor3f(xx*xx, yy*yy, 1.0f);//triangle color RGB
      glVertex3f( xx, yy, zz);
      glVertex3f( xxp,  yyp, zzp);
    }
    glEnd();
  }
  glPopMatrix();

  glXSwapBuffers(dpy, win);

  //usleep(1);

  angle += 2;
  if(360 == angle) angle = 0;

  pos += 0.005;
  if(pos >= 1.0) pos = 0.0;
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
