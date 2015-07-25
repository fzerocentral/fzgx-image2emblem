//This class creates the binary output for the emblem data


class MakeEmblem{  
  
    void LoadFile(){
            
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
  
  void Header() {
  
    //Create a "complex" file name. It's a mix of the format standard file with a
    //real filename in it, as well as a reference to the convert's version number.
    //Format: fze020_ + (filename) + (date in hex (3 characters)) + .dat
    
    String[] prefix = loadStrings("data/prefix");
    String outputName;
    
    try{
      outputName = prefix[0] + inputFileName;
      
    //An error occurs when the preix file is blank due to there being an array length of null
    } catch (Exception e) {
      outputName = inputFileName;
    }
        
    //If prefix + filename is not 10 characters long, append _ until it is
    while(outputName.length() < 16){
      outputName += "_";
    }
    
    outputName = outputName.substring(0, 16);
    outputFileName = "fze020_" + outputName + "_" + hex(timeStamp.CurrentDate(), 3) + ".dat";

    
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
    
  void FormatColorData(){
    
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


  void EmblemData() {
    
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
  
  
  void BannerData(){
      
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
  
  
  void OutputFile(){
 
    CropEmblemEdges();
    
    //Add comment to the last bytes of the file. HAS TO BE 48 BYTES LONG.
    String comment = "Emblem created  w/ ImageToEmblem" + versionNumber + "  " + timeStamp.CurrentDate();
    
   /* 0x0000 */ emblemFile = header;
   /* 0x00A0 */ emblemFile = concat(emblemFile, bannerData);
   /* 0x18A0 */ emblemFile = concat(emblemFile, loadBytes("data/resources/emblem_icon"));
   /* 0x20A0 */ emblemFile = concat(emblemFile, emblemImageData);
   /* 0x40A0 */ emblemFile = concat(emblemFile, new byte[0x2000 - 0xA0]); //Null at the end, minus size of header data
   /* 0x6020 */ emblemFile = concat(emblemFile, comment.getBytes());
   /* 0x6050 */ emblemFile = concat(emblemFile, new byte[0x10]);

   
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


  void MakeChecksum(){
    
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
    emblemFile[0x40] = byte(checksum >>> 8);
    emblemFile[0x41] = byte(checksum);
    
    //Debug
    //println("Checksum: " + hex(checksum, 4));
  }
  
  
  
  void CropEmblemEdges(){
    
    //Upper left corner
    if (cropBorderArea[0]){
      imageDataRaw[0] = byte(0);
      imageDataRaw[1] = byte(0);
    }
    
    //Upper right corner
    if (cropBorderArea[2]){
      imageDataRaw[63*2] = byte(0);
      imageDataRaw[63*2+1] = byte(0);
    }
    
    //Lower left corner
    if (cropBorderArea[6]){
      imageDataRaw[8190-126] = byte(0);
      imageDataRaw[8191-126] = byte(0);
    }
    
    //Lower right corner
    if (cropBorderArea[8]){
      imageDataRaw[8190] = byte(0);
      imageDataRaw[8191] = byte(0);
    }
    
    //Left side
    if (cropBorderArea[3]){
      for (int i = 64*2; i < 8192 - 64*2; i++){
        if (i % 128 == 0){
          imageDataRaw[i] = byte(0);
          imageDataRaw[i+1] = byte(0);
        } 
      } 
    }
    
    //Right side
    if (cropBorderArea[5]){
      for (int i = 64*2; i < 8192 - 64*2; i++){
        if (i % 128 == 126){
          imageDataRaw[i] = byte(0);
          imageDataRaw[i+1] = byte(0);
        } 
      } 
    }
    
    //Top
    if (cropBorderArea[1]){
      for (int i = 1*2; i < 63*2; i++){
        imageDataRaw[i] = byte(0);
      }
    }
    
    //Bottom
    if (cropBorderArea[7]){
      for (int i = 8064 + 2; i < 8192 - 2; i++){
        imageDataRaw[i] = byte(0);
      }
    }
    

  }

}
