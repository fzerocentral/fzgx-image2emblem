//Code originally by Yoshifan - @Yoshifan28
//Code ported by Raphael Tetreault - @StarkNebula
//Super special thanks to Ralf of gc-forever for the checksum algorithm

//This code can be cleaned further ;_;

/* IMPORT CUSTOM CLASSES */
TimeStamp timeStamp = new TimeStamp();
MakeEmblem make = new MakeEmblem();
GUI gui = new GUI();

/* INITIALIZE VARIABLES */
//6 character limit
String versionNumber = "v1.00_";
String outputFileName = null; //Gets filled later

//PImage to hold image data
PImage emblemImage = new PImage();
PImage fzcLogo = new PImage();

//Bytes for file
byte[] header = new byte[0x60];

byte[] imageDataRaw = new byte[0x2000];
byte[] emblemImageData = new byte[0x2000];

byte[] bannerData = new byte[0x1800];
byte[] bannerMiniEmblem = new byte[0x800];

//Byte array for final output file
byte[] emblemFile = new byte[0];

//Arrays to store pixel data as it gets converted
byte emblemColorA, emblemColorR, emblemColorG, emblemColorB;
byte emblemByteAR, emblemByteGB;


/* GUI VARIABLES */

boolean[] cropBorderArea = new boolean[9];
//[0][1][2]
//[3][4][5] - [4] is an all On/Off switch
//[6][7][8]

boolean[] region = new boolean[3];
// 1 - US
// 2 - JP
// 3 - PAL

//Colors
//Combination makes mid-tone purple
int red = 160;
int green = 64;
int blue = 256;
color isSelectedColor = color(red, green, blue);
color isNotSelectedColor = color(255); //white
color strokeColor = color(red/2, green/2, blue/2);
color textColor = color(0);

//Emblem crop area box sizes, etc.
int cropCornerX = 150;
int cropCornerY = 50;
int cropBoxSize = 80;
int cropProtrusion = 80/4; //for center

//Region checkboxes size, etc
int checkboxStartX = 50;
int checkboxStartY = 180;
int checkboxSize = 30;
int checkboxSpacing = checkboxSize + 10;

//Text
int textSize = 18;
float textAlign = 1.2; //hacky little value to help place text
int indentEmblemText = 30;

//Size of window
int screenWidth = 450;
int screenHeight = 350;

/* LOAD FILE VARIABLES*/
boolean loadFileFunctionHasNotRun = true;
boolean fileWasLoaded;
  
/* HEADER VARIABLES */
byte[] gameTitle;
byte[] gameID;
byte[] dataName;

byte[] timeStampAsText;
String timeStampInInts;
byte[] timeStampInSecondsInHex;

byte[] constant_FF02 = { (byte) 0xFF, (byte) 0x02 };
byte[] constant_0401 = { (byte) 0x04, (byte) 0x01 };
byte[] constant_0003FFFF_00000004      = { (byte) 0x00, (byte) 0x03, (byte) 0xff, (byte) 0xff, (byte) 0x00, (byte) 0x00, (byte) 0x00, (byte) 0x04 };
byte[] constant_00000060_00020003_0400 = { (byte) 0x00, (byte) 0x00, (byte) 0x00, (byte) 0x60, (byte) 0x00, (byte) 0x02, (byte) 0x00, (byte) 0x03, (byte) 0x04, (byte) 0x00 };

//BannerData()
int iconPixel = 0;



void setup() {
  frameRate(30);
  
  //Load blank banner data on start
  bannerData = loadBytes("data/resources/emblem_banner_blank");
  
  fzcLogo = loadImage("data/resources/fzcLogo.png");
  size(screenWidth, screenHeight);
  background(255, 242, 225);
  //Add bg image
  //PImage bg = loadImage("data/resources/bg001.png");
  //image(bg, 0, 0);
  image(fzcLogo, 20, 20, 110, 110);
  gui.GUI();
  
  //Hackish. Run click once to display graphics, otherwise screen is blank
  mousePressed();
}



//Need to have draw() or else key inputs do not work
void draw(){}


void keyPressed(){
  //if SPACEBAR is hit
  if (key == ' '){
    MakeFile();
  }
}


void MakeFile(){  
  //Run through functions to build file
  make.LoadFile();
  
  //if file was loaded, run further code
  if (fileWasLoaded){
    make.Header();
    make.FormatColorData();
    make.BannerData();
    make.EmblemData();
    make.OutputFile();
  }
}


//For some reason does not like running from class, so I left it in the main sketch
void LoadEmblemImage(File _selectedImage) {
  
  loadFileFunctionHasNotRun = false;
  
  if (_selectedImage == null){
    println("Image not loaded. Did you cancel?");
    fileWasLoaded = false;
        
  } else {
    try {
      emblemImage = loadImage(_selectedImage.getAbsolutePath());
      fileWasLoaded = true;
      
      if (emblemImage.pixels.length != (64*64)){
        println("Image is not 64*64! Process aborted!");
        fileWasLoaded = false;
      }

    } catch (Exception e){
      fileWasLoaded = false;
      println("An exception occured");
    }
  }
}

