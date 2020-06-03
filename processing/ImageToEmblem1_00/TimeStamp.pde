class TimeStamp {

  int CurrentDate() {
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
    println(years + "y " + months + "m " + days + "d " + hours + "h " + minutes + "m " + seconds + "s ");
    int total = (((totalDaysInYears + totalDaysInMonths + days) * 24 + hours) * 60 + minutes) * 60 + seconds;
    //println(total);
    //println(hex(total));
    //println(hex(total).getBytes());

    return total;
  }
}

