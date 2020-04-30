import peasy.*;

PeasyCam cam;
PVector[][] globe;
int total = 20;

void setup(){
  size(600,600,P3D);
  cam = new PeasyCam(this,200);
  colorMode(HSB);
  globe  = new PVector[total+1][total+1];
}

void draw(){
  background(0);
  fill(255);
  noStroke();
  lights();
  //translate(width/2,height/2);
  //sphere(200);
  
  for(int i = 0; i<total+1; i++){
    float lon = map(i,0, total, -PI, PI);
    for(int j = 0; j<total+1; j++){
      float r = 100;
      float lat =map(j,0,total,-PI, PI);
      float x = r * sin(lon)*cos(lat);
      float y = r * sin(lon)*sin(lat);
      float z = r * cos(lon);
      globe[i][j] =new PVector(x,y,z);
      //PVector v = PVector.random3D();
      //v.mult(15);
      //globe[i][j].add(v);
      //stroke(255);
      //strokeWeight(4);
      //point(x,y,z);
    }
  }
  for(int i = 0; i<total; i++){
      //float hu = map(i,0,total,0,255*6);
    beginShape(TRIANGLE_STRIP);
    for(int j = 0; j<total+1; j++){
      float hu = map(j,0,total,0,255*6);
      fill(hu%255,255,255);
      PVector v1 = globe[i][j]; 
      vertex(v1.x,v1.y,v1.z);
      PVector v2 = globe[i+1][j]; 
      vertex(v2.x,v2.y,v2.z);
    }
    endShape();
  }  
  
}
