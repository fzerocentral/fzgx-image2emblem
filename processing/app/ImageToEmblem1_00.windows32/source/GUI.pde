class GUI{
  
  //Setup GUI
  void GUI(){
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
  
  
  boolean cropButton(int _x, int _y, int _w, int _h, boolean buttonBoolean, color _isSelectedColor){
      
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
  
  boolean regionButton(int _x, int _y, int _w, int _h, boolean _thisRegion, color _isSelectedColor){
    
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
  void mousePressed(){
    
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
  
      //////////////////
     //DEBUG
    //Check the status of each boolean for cut area
    for (int i = 0; i < cropBorderArea.length; i++){
      println("Cut " + i + ": " + cropBorderArea[i]);
    }
    
    //Check the status of each boolean for region
    for (int i = 0; i < region.length; i++){
      println("Region " + i + ": " + region[i]);
    }//
  }
