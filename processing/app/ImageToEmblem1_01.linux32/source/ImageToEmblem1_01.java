import processing.core.*; 
import processing.data.*; 
import processing.event.*; 
import processing.opengl.*; 

import java.util.HashMap; 
import java.util.ArrayList; 
import java.io.File; 
import java.io.BufferedReader; 
import java.io.PrintWriter; 
import java.io.InputStream; 
import java.io.OutputStream; 
import java.io.IOException; 

public class ImageToEmblem1_01 extends PApplet {

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
String versionNumber = "v1.01";
String inputFileName;
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
int isSelectedColor = color(red, green, blue);
int isNotSelectedColor = color(255); //white
int strokeColor = color(red/2, green/2, blue/2);
int textColor = color(0);

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
float textAlign = 1.2f; //hacky little value to help place text
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



public void setup() {
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
public void draw(){}


public void keyPressed(){
  //if SPACEBAR is hit
  if (key == ' '){
    MakeFile();
  }
}


public void MakeFile(){  
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
public void LoadEmblemImage(File _selectedImage) {
  
  //Refer to LoadFile function in MakeEmblem class
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
      
      //Get input file's name
      inputFileName = _selectedImage.getAbsolutePath();
      //println(inputFileName);

      String[] splitPathName = split(inputFileName, '/');
      //println(splitPathName);
      //println(splitPathName.length);

      inputFileName = splitPathName[splitPathName.length - 1];
      println(inputFileName);

      splitPathName = split(inputFileName, '.');
      
      inputFileName = splitPathName[0];

    } catch (Exception e){
      fileWasLoaded = false;
      println("An exception occured");
    }
  }
}
class GUI{
  
  //Setup GUI
  public void GUI(){
   //Display background image
   //image(loadImage("efuzerocenta.png"), 0, 0);
   
   //Text
   textSize(18);
   
   //Strokes around rectangles
   strokeWeight(1);
   stroke(strokeColor);
   
   //Initiallize all values of crop area to true (AKA - do crop all edges)
    for (int i = 0; i < cropBorderArea.length; i++){
       cropBorderArea[i] = true;
    }
    
   //Set region[0] (US) ON
   region[0] = true;
   
   //DISPLAY UI TEXT//
   fill(textColor);
    
   text("Crop Emblem Border", cropCornerX + indentEmblemText, cropCornerY - textSize/2);
    
   text("Region", checkboxStartX + textSize/4, checkboxStartY - textSize);
   text("US", checkboxStartX + checkboxSize*textAlign, checkboxStartY + textSize*textAlign);
   text("JP", checkboxStartX + checkboxSize*textAlign, checkboxStartY + textSize*textAlign + checkboxSpacing);
   text("PAL", checkboxStartX + checkboxSize*textAlign, checkboxStartY + textSize*textAlign + checkboxSpacing*2);
    
   textAlign(CENTER);
   text("Press Space To Load Image", screenWidth/2, screenHeight - textSize);
  }
  
  
  //Draw needs to be present in order to refresh graphics even if it draws nothing itside the function
  //void draw(){}
  
  //Note for myself
  // Where MousePressed(){} goes in standalone sketch //
  
  
  public boolean cropButton(int _x, int _y, int _w, int _h, boolean buttonBoolean, int _isSelectedColor){
      
    if (mouseX > _x && mouseX < _x + _w){
      if (mouseY > _y && mouseY < _y + _h){
        buttonBoolean = !buttonBoolean;
      }
    }
    
    if (buttonBoolean){
      fill(_isSelectedColor);
    } else {
      fill(255);
    }
      
    rect(_x, _y, _w, _h);
    return buttonBoolean;
  }
  
  public boolean regionButton(int _x, int _y, int _w, int _h, boolean _thisRegion, int _isSelectedColor){
    
    if (mouseX > _x && mouseX < _x + _w){
      if (mouseY > _y && mouseY < _y + _h){
        //turn all regions off
        for (int i = 0; i < region.length; i++){
         region[i] = false;
        }
        //turn this region on
        _thisRegion = true;
      }
    }
    
    if (_thisRegion){
      fill(_isSelectedColor);
    } else {
      fill(255);
    }
      
    rect(_x, _y, _w, _h);
    return _thisRegion;
  }
}



//This basically serves as draw()
  public void mousePressed(){
    
    //Visual feedback for: Crop sides or corners of image
    //Setting the booling on will cut it (when colored), leaving it will let it pass as full 64x64
    
    cropBorderArea[0] = gui.cropButton(cropCornerX + cropBoxSize*0, cropCornerY, cropBoxSize, cropBoxSize, cropBorderArea[0], isSelectedColor);
    cropBorderArea[1] = gui.cropButton(cropCornerX + cropBoxSize*1, cropCornerY, cropBoxSize, cropBoxSize, cropBorderArea[1], isSelectedColor);
    cropBorderArea[2] = gui.cropButton(cropCornerX + cropBoxSize*2, cropCornerY, cropBoxSize, cropBoxSize, cropBorderArea[2], isSelectedColor);
    
    cropBorderArea[3] = gui.cropButton(cropCornerX + cropBoxSize*0, cropCornerY + cropBoxSize*1, cropBoxSize, cropBoxSize, cropBorderArea[3], isSelectedColor);
    //Hackish. Left simply to keep BG area white.
    cropBorderArea[4] = gui.cropButton(cropCornerX + cropBoxSize*1, cropCornerY + cropBoxSize*1, cropBoxSize, cropBoxSize, cropBorderArea[4], isNotSelectedColor);
    cropBorderArea[5] = gui.cropButton(cropCornerX + cropBoxSize*2, cropCornerY + cropBoxSize*1, cropBoxSize, cropBoxSize, cropBorderArea[5], isSelectedColor);
    
    cropBorderArea[6] = gui.cropButton(cropCornerX + cropBoxSize*0, cropCornerY + cropBoxSize*2, cropBoxSize, cropBoxSize, cropBorderArea[6], isSelectedColor);
    cropBorderArea[7] = gui.cropButton(cropCornerX + cropBoxSize*1, cropCornerY + cropBoxSize*2, cropBoxSize, cropBoxSize, cropBorderArea[7], isSelectedColor);
    cropBorderArea[8] = gui.cropButton(cropCornerX + cropBoxSize*2, cropCornerY + cropBoxSize*2, cropBoxSize, cropBoxSize, cropBorderArea[8], isSelectedColor);
    
    //Little box in center to crop area, simply an outline as fill is 100% alpha
    fill(red, green, blue, 64);
    rect(cropCornerX - cropProtrusion + cropBoxSize*1, cropCornerY + cropBoxSize*1 - cropProtrusion, cropBoxSize + cropProtrusion*2, cropBoxSize + cropProtrusion*2);
    fill(255);
    
    //Hackish. Run this code twice to ensure color updates properly
    for (int i = 0; i < 2; i++){
      region[0] = gui.regionButton(checkboxStartX, checkboxStartY, checkboxSize, checkboxSize, region[0], isSelectedColor);
      region[1] = gui.regionButton(checkboxStartX, checkboxStartY + checkboxSpacing, checkboxSize, checkboxSize, region[1], isSelectedColor);
      region[2] = gui.regionButton(checkboxStartX, checkboxStartY + checkboxSpacing*2, checkboxSize, checkboxSize, region[2], isSelectedColor);
    }
  
      /*/////////////////
     //DEBUG
    //Check the status of each boolean for cut area
    for (int i = 0; i < cropBorderArea.length; i++){
      println("Cut " + i + ": " + cropBorderArea[i]);
    }
    
    //Check the status of each boolean for region
    for (int i = 0; i < region.length; i++){
      println("Region " + i + ": " + region[i]);
    }/*/
  }
//This class creates the binary output for the emblem data


class MakeEmblem{  
  
    public void LoadFile(){
            
    //Make image array null each passthrough, enabling the while loop to delay further functions from being run until file is selected
    emblemImage = null;
    fileWasLoaded = false;
    loadFileFunctionHasNotRun = true;
    
    //Load image file. (display text, function that loads file) 
    selectInput("Load Emblem Image 64x64", "LoadEmblemImage");
    
    //Until input is selected, delay further progress, otherwise the proceeding code would run without a source file
    while (emblemImage == null && loadFileFunctionHasNotRun){
      //delay every run until file is loaded 
      delay(100); //Delay by 100ms
    } 
    
    //Added another delay because the selectInput method is not performed in the same thread.
    //Essentially, delaying the code in this thread gives the CPU time to sync the selectedInput
    //file to this one, enabling the code to not prematurely void further code as it is seen as null
    //when it gets processing it down the line. Without it, the program may "randomly" skip input files, 
    //resulting in files not being converted.
    delay (100);
  }
  
  public void Header() {
  
    //Create a "complex" file name. It's a mix of the format standard file with a
    //real filename in it, as well as a reference to the convert's version number.
    //Format: fze + (_filename_) + (date in hex) + (version number) + .dat
    
    String[] prefix = loadStrings("data/prefix");
    String outputName;
    
    try{
      outputName = prefix[0] + inputFileName;
      
    //An error occurs when the preix file is blank due to there being an array length of null
    } catch (Exception e) {
      outputName = inputFileName;
    }
        
    //If prefix + filename is not 10 characters long, append _ until it is
    while(outputName.length() < 10){
      outputName += "_";
    }
    
    outputName = outputName.substring(0, 14 - versionNumber.length());
    outputFileName = "fze" + "_" + outputName + "_" + hex(timeStamp.CurrentDate(), 8) + versionNumber + ".dat";

    
    //Set region. Not sure if it actually matters or not
    if (region[0]){ //US
      gameID    = "GFZE8P".getBytes();
    } else if (region[1]){ //JP
      gameID    = "GFZJ8P".getBytes();
    } else if (region[2]){ //PAL
      gameID    = "GFZP8P".getBytes();
    }
    
    gameTitle = "F-ZERO GX".getBytes();
    dataName  = outputFileName.getBytes();
  
    //Get current time and make timestamp string out of it, convert string it to bytes
    timeStampAsText = (nf(year() - 2000, 2) + "/" + nf(month(), 2) + "/" + nf(day(), 2) + " " + nf(hour(), 2) + ":" + nf(minute(), 2)).getBytes();
    //Convert time into hexidecimal and string simultaneously
    timeStampInInts = hex(timeStamp.CurrentDate());
    //Pick apart the each byte of the timestamp string and write it as a byte
    timeStampInSecondsInHex = new byte[] { (byte) unhex(timeStampInInts.substring(0, 2)), (byte) unhex(timeStampInInts.substring(2, 4)), (byte) unhex(timeStampInInts.substring(4, 6)), (byte) unhex(timeStampInInts.substring(6, 8)) };
  

    //Concatonate header in proper order
    //new byte[x] is typically nulls, if not portions of header that can't be cemented yet
    /* 0x00 */ header = gameID;
    /* 0x06 */ header = concat(header, constant_FF02);
    /* 0x08 */ header = concat(header, dataName);
    /* 0x27 */ header = concat(header, new byte[1]); //NULL
    /* 0x28 */ header = concat(header, timeStampInSecondsInHex);
    /* 0x2C */ header = concat(header, constant_00000060_00020003_0400);
    /* 0x36 */ header = concat(header, new byte[2] ); //Copy Count
    /* 0x38 */ header = concat(header, constant_0003FFFF_00000004);
    /* 0x40 */ header = concat(header, new byte[2]); //Checksum is built later, null for now
    /* 0x42 */ header = concat(header, constant_0401);
    /* 0x44 */ header = concat(header, gameTitle);
    /* 0x4D */ header = concat(header, new byte[23]); //NULL
    /* 0x64 */ header = concat(header, timeStampAsText);
    /* 0x72 */ header = concat(header, new byte[46]); //NULL
  
    //Save header as file. For debugging
    //saveBytes("data/emblem/headerTest.emb", header);
  }
    
  public void FormatColorData(){
    
    //Loop through 4096 times. Pixel data is 8192 bytes, but the following function processes 2 bytes at a time
    for (int i = 0; i < 0x1000; i++) {
      //Get pixel data and trim it to FZGX's compressed 2 byte format using bitshifts and casting to byte (bytes are 8 bits, pixels are 32 bits, cut to 16 bit length)
      //After bitshift, since bytes are signed, bits need to be flipped in order for divisor operation to work (ie ubyte 255 = byte -1. byte (-1 / 8) = 0 rather than (255 / 8) = 31.
      emblemColorA = (byte) ((emblemImage.pixels[i] >>> 24 & 0xff) / 128); //divided by 128 makes anything alpha 127 or less transparent, anything 128 or more opaque, resulting in a 1 bit alpha signifier
      emblemColorR = (byte) ((emblemImage.pixels[i] >>> 16 & 0xff) / 8); //divided by 8 to have R take up 5 bits
      emblemColorG = (byte) ((emblemImage.pixels[i] >>>  8 & 0xff) / 8); //divided by 8 to have G take up 5 bits
      emblemColorB = (byte) ((emblemImage.pixels[i] >>>  0 & 0xff) / 8); //divided by 8 to have B take up 5 bits
  
      //Remove 3 MSD bits leftover from previous stage, resulting in clean 5-bit numbers. Makes bitshifting easy later
      emblemColorR ^= (emblemColorR & (7 << 5)); //Erase 111 (5) in binary from leftmost bits
      emblemColorG ^= (emblemColorG & (7 << 5));
      emblemColorB ^= (emblemColorB & (7 << 5));
  
      //Debug
      //println("emblemColorR :" + binary(emblemColorR));
      //println("emblemColorG :" + binary(emblemColorG));
      //println("emblemColorB :" + binary(emblemColorB));
  
      //Splice ARGB into 2 bytes
      //AR holds Alpha, Red, and 2/5 Green bits
      //GB holds 3/5 Green bits, Blue bits
      if (emblemColorA == 1) {
        emblemByteAR = (byte) (emblemColorG >>> 3 | emblemColorR << 2 | emblemColorA << 7);
        emblemByteGB = (byte) (emblemColorB | (emblemColorG << 5));
  
      //If alpha is 0, make pixel true alpha. GX doesn't recognize alpha 0 if colors are not floored to 0
      } else {
        emblemByteAR = (byte) 0;
        emblemByteGB = (byte) 0;
      }
  
      //Store bytes in array with proper 1-2 intervals for each byte
      imageDataRaw[i*2] = emblemByteAR;
      imageDataRaw[i*2+1] = emblemByteGB;
    }
  }


  public void EmblemData() {
    
    //Cut edges if booleans are set (from GUI)
    CropEmblemEdges();
    
    //Format pixel data to 4x4 chunks in 16x16 grid
    for (int blockRow = 0; blockRow < 16; blockRow++) {
      for (int blockColumn = 0; blockColumn < 16; blockColumn++) {
        for (int chunkHeight = 0; chunkHeight < 4; chunkHeight++) {
          for (int chunkWidth = 0; chunkWidth < 4; chunkWidth++) {
            emblemImageData [((blockRow*256 + blockColumn*16 + chunkHeight*4 + chunkWidth)*2 + 0)] = imageDataRaw [((chunkWidth + (chunkHeight * 64) + (blockColumn * 4) + (blockRow * 256))*2 + 0)];
            emblemImageData [((blockRow*256 + blockColumn*16 + chunkHeight*4 + chunkWidth)*2 + 1)] = imageDataRaw [((chunkWidth + (chunkHeight * 64) + (blockColumn * 4) + (blockRow * 256))*2 + 1)];
            //println("New: " + ((blockRow*256 + blockColumn*16 + chunkHeight*4 + chunkWidth)*2 + 0) + " loaded old: " + ((chunkWidth + (chunkHeight * 64) + (blockColumn * 4) + (blockRow * 256))*2 + 0));
            //println("New: " + ((blockRow*256 + blockColumn*16 + chunkHeight*4 + chunkWidth)*2 + 1) + " loaded old: " + ((chunkWidth + (chunkHeight * 64) + (blockColumn * 4) + (blockRow * 256))*2 + 1));
          }
        }
      }
    }
    //println("Done Sorting");
    //saveBytes("data/emblem/emblemDataTest.emb", emblemImageData);
  }
  
  
  public void BannerData(){
      
    //Format pixel data to 4x4 chunks in 8x8 grid
    for (int blockRow = 0; blockRow < 8; blockRow++) {
      for (int blockColumn = 0; blockColumn < 8; blockColumn++) {
        for (int chunkHeight = 0; chunkHeight < 4; chunkHeight++) {
          for (int chunkWidth = 0; chunkWidth < 4; chunkWidth++) {
            //*4 on far right end of code means skip every other pixel, turns 64*64 to 32*32
            bannerMiniEmblem [((blockRow*128 + blockColumn*16 + chunkHeight*4 + chunkWidth)*2 + 0)] = imageDataRaw [((chunkWidth + (chunkHeight * 64) + (blockColumn * 4) + (blockRow * 256))*4 + 0)];
            bannerMiniEmblem [((blockRow*128 + blockColumn*16 + chunkHeight*4 + chunkWidth)*2 + 1)] = imageDataRaw [((chunkWidth + (chunkHeight * 64) + (blockColumn * 4) + (blockRow * 256))*4 + 1)];
          }
        }
      }
    }
    
    //Splice the two graphics together. If run multiple times, emblem data simply gets rewritten.
    //Keep track of pixel offset the easy way. Resets every pass thorugh.
    iconPixel = 0;
    //for length of banner data, ever 256 - 384 pixel is a minified version of emblem data, created above. Could be better (have it be an average of the color rather than skip)
    for (int i = 0; i < 0x1800; i++){
      if (i % 0x300 >= 0x200){
        bannerData[i] = bannerMiniEmblem[iconPixel];
        iconPixel ++;
        //Debug. Purple/magenta color
        //bannerBase[i] = (byte) (-65535/7);
      } 
    }
  }
  
  
  public void OutputFile(){
 
    CropEmblemEdges();
    
   /* 0x0000 */ emblemFile = header;
   /* 0x00A0 */ emblemFile = concat(emblemFile, bannerData);
   /* 0x18A0 */ emblemFile = concat(emblemFile, loadBytes("data/resources/emblem_icon"));
   /* 0x20A0 */ emblemFile = concat(emblemFile, emblemImageData);
   /* 0x40A0 */ emblemFile = concat(emblemFile, new byte[0x2000 - 0x60]); //Null at the end, minus size of header data
   
   //Old debug stuff
   //println("Header: " + hex(header.length));
   //println("fzgxEmblemBanner: " + hex(bannerData.length));
   //println("Icon: " + hex(loadBytes("data/icon.emb.dat").length));
   //println("emblemImageData: " + hex(emblemImageData.length));
   
   MakeChecksum();
   
   //Save File
   saveBytes("emblemOutput/" + outputFileName + ".gci", emblemFile);
   println("File succesfully converted!");
   println("emblemOutput/" + outputFileName + ".gci was created!");
}


  public void MakeChecksum(){
    
    int checksum = 0xFFFF;
    int generatorPolynomial = 0x8408;
    
    //for all data after checksum
    for (int i = 0x42; i < emblemFile.length; i ++){
      
      checksum = checksum ^ ((emblemFile[i]) & 0xFF);
      
      //For each bit in byte
      for (int j = 8; j> 0; j--){
        
        if ((checksum & 1) == 1){
          checksum  = (checksum >>> 1) ^ generatorPolynomial;
  
        } else {
          checksum  = (checksum >>> 1);
        }
      }
    }
    
    //Final operation: flip all bits
    checksum ^= 0xFFFF; 
  
    //Overwrite null/old checksum data with new checksum
    emblemFile[0x40] = PApplet.parseByte(checksum >>> 8);
    emblemFile[0x41] = PApplet.parseByte(checksum);
    
    //Debug
    //println("Checksum: " + hex(checksum, 4));
  }
  
  
  
  public void CropEmblemEdges(){
    
    //Upper left corner
    if (cropBorderArea[0]){
      imageDataRaw[0] = PApplet.parseByte(0);
      imageDataRaw[1] = PApplet.parseByte(0);
    }
    
    //Upper right corner
    if (cropBorderArea[2]){
      imageDataRaw[63*2] = PApplet.parseByte(0);
      imageDataRaw[63*2+1] = PApplet.parseByte(0);
    }
    
    //Lower left corner
    if (cropBorderArea[6]){
      imageDataRaw[8190-126] = PApplet.parseByte(0);
      imageDataRaw[8191-126] = PApplet.parseByte(0);
    }
    
    //Lower right corner
    if (cropBorderArea[8]){
      imageDataRaw[8190] = PApplet.parseByte(0);
      imageDataRaw[8191] = PApplet.parseByte(0);
    }
    
    //Left side
    if (cropBorderArea[3]){
      for (int i = 64*2; i < 8192 - 64*2; i++){
        if (i % 128 == 0){
          imageDataRaw[i] = PApplet.parseByte(0);
          imageDataRaw[i+1] = PApplet.parseByte(0);
        } 
      } 
    }
    
    //Right side
    if (cropBorderArea[5]){
      for (int i = 64*2; i < 8192 - 64*2; i++){
        if (i % 128 == 126){
          imageDataRaw[i] = PApplet.parseByte(0);
          imageDataRaw[i+1] = PApplet.parseByte(0);
        } 
      } 
    }
    
    //Top
    if (cropBorderArea[1]){
      for (int i = 1*2; i < 63*2; i++){
        imageDataRaw[i] = PApplet.parseByte(0);
      }
    }
    
    //Bottom
    if (cropBorderArea[7]){
      for (int i = 8064 + 2; i < 8192 - 2; i++){
        imageDataRaw[i] = PApplet.parseByte(0);
      }
    }
    

  }

}
class TimeStamp {

  public int CurrentDate() {

    int seconds = second();
    int minutes = minute();
    int hours = hour();
    int days = day();
    int months = month();
    int years = year();

    int totalDaysInYears = (years - 2000) * 365;
    int totalDaysInMonths = 0;
    
    //Calculate Days in Months
    switch (months) {
      //Past November
    case 12:
      totalDaysInMonths += 30;
      //println("In December");

      //Past October
    case 11:
      totalDaysInMonths += 31;  
      //println("In November");

      //Past September
    case 10:
      totalDaysInMonths += 30;  
      //println("In October");

      //Past August
    case 9:
      totalDaysInMonths += 31;  
      //println("In September");

      //Past July
    case 8:
      totalDaysInMonths += 31;  
      //println("In August");

      //Past June
    case 7:
      totalDaysInMonths += 30;  
      //println("In June");

      //Past May
    case 6:
      totalDaysInMonths += 31;  
      //println("In July");

      //Past April
    case 5:
      totalDaysInMonths += 30;  
      //println("In May");

      //Past March
    case 4:
      totalDaysInMonths += 31;  
      //println("In April");

      //Past February
    case 3:
      totalDaysInMonths += 28;
      int totalDaysInYearsWhileLoop = years - 2000;
      
      while (totalDaysInYearsWhileLoop >= 4) {
        totalDaysInYearsWhileLoop -= 4;
        //println (totalDaysInYearsWhileLoop);
        totalDaysInMonths += 1;
      }
      //println("In March");

      //Past January
    case 2:
      totalDaysInMonths += 31;  
      //println("In February");

      //Currently is January
    case 1:
      //nothing to add here
      //println("In January");
      break;

    default:
      //totalMonths = 0; 
      break;
    }
    
    //println(totalDaysInMonths);
    //println(years + "y " + months + "m " + days + "d " + hours + "h " + minutes + "m " + seconds + "s ");
    int total = (((totalDaysInYears + totalDaysInMonths + days) * 24 + hours) * 60 + minutes) * 60 + seconds;
    //println(total);
    //println(hex(total));
    //println(hex(total).getBytes());
    return total;
  }
}
  static public void main(String[] passedArgs) {
    String[] appletArgs = new String[] { "ImageToEmblem1_01" };
    if (passedArgs != null) {
      PApplet.main(concat(appletArgs, passedArgs));
    } else {
      PApplet.main(appletArgs);
    }
  }
}
