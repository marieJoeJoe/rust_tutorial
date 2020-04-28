#include <stdio.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <>
#include <>
#include <>


int main(int agrgc,char argv){
  int sock;
  struct sockaddr_in server;

  int mysock;
  char buff[1024];
  int rval;

  sock = socket(AF_INET,SOCK_STREAM,O);
  if(sock < 0) {
    perror("Failed to create socket");
    exit(1);
  }

  server.sin_family = AF_INET;
  server.sin_addr.s_addr  = INADDR_ANY;
  server.sin_Port = 5000;

  if(bind(sock,(struct sockaddr *)&server,sizeof(server))) {
    perror("bind failed!");
    exit(1);
 
  }

}
