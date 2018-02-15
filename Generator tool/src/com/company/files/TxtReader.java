package com.company.files;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 1/11/18   ==
* =====================================================================
* ==      Project: Generator tool    ==
* =====================================================================

*/

import com.company.screen.Menu;
import java.io.File;
import java.util.ArrayList;
import java.util.Scanner;

public class TxtReader {

    private File file;
    private ArrayList<Float> data;
    private int v;

    public TxtReader(File file){
        this.file = file;
        data = new ArrayList<Float>();
    }

    public void read(){


        try {
            Scanner reader = new Scanner(file);

            while (reader.hasNextLine()){

                String k = reader.nextLine();


                if (k.length()>1){
                    if (!(k.charAt(0)=='#'&&k.charAt(1)=='#')){

                        if ((k.charAt(0)=='['&&(k.charAt(1)=='1'))){
                            //version 1
                             v = 1;
                        }else if (v == 1||v==2||v==3||v==4){
                            readLineV1(k);

                        }else if (k.charAt(0)=='['&&k.charAt(1)=='2'){
                            v = 2;
                        }else if (k.charAt(0)=='['&&k.charAt(1)=='3'){
                            v = 3;
                        }else if (k.charAt(0)=='['&&k.charAt(1)=='4'){
                            v = 4;
                    }

                    }
                }

            }

            if (v==1){
                System.out.println("    CORRECT: Initial condition file successfully loaded");

                System.out.println("        File contains "+data.size()/8+" bodies");
                Menu.iniLoaded = true;

            }else if (v==2){
                System.out.println("    CORRECT: Initial condition file successfully loaded");

                System.out.println("        File contains "+data.size()/7+" bodies");
                Menu.iniLoaded = true;

            }else if (v==3){
                System.out.println("    CORRECT: Initial condition file successfully loaded");

                System.out.println("        File contains "+data.size()/11+" bodies");
                Menu.iniLoaded = true;

            }else if (v==4){
                System.out.println("    CORRECT: Initial condition file successfully loaded");

                System.out.println("        File contains "+data.size()/7+" bodies");
                Menu.iniLoaded = true;

            }else {
                System.out.println("ERROR: FILE FORMAT NOT SUPPORTED");
            }

        }catch (Exception e){
            System.out.println("    ERROR: Exception while reading file. ");
            System.out.println("    "+e.getMessage());
        }

       // for (float b : data){
        //    System.out.println(b);
        //}



    }

    private void readLineV1(String k){
        for (int i = 0; i < k.length(); i++){
            if (k.charAt(i)=='-'||k.charAt(i)=='0'||k.charAt(i)=='1'||k.charAt(i)=='2'||k.charAt(i)=='3'||k.charAt(i)=='4'||k.charAt(i)=='5'||k.charAt(i)=='6'||k.charAt(i)=='7'||k.charAt(i)=='8'||k.charAt(i)=='9'){
                int a = getNumberV1(i, k);

                if (a == k.length()-1){
                    data.add(Float.parseFloat(k.substring(i)));
                    i = k.length()-1;
                }else {
                    data.add(Float.parseFloat(k.substring(i, a)));
                    i = a-1;
                }



            }
        }
    }

    private int getNumberV1(int a, String k){
        for (int i = a; i < k.length(); i++){
            if (k.charAt(i)==' '||k.charAt(i)==','){
                return i;
            }
        }
        return k.length()-1;
    }

    public ArrayList<Float> getData(){
        return  data;
    }

    public int getV(){
        return v;
    }
}
