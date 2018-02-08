package com.company.screen;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 1/11/18   ==
* =====================================================================
* ==      Project: Generator tool    ==
* =====================================================================

*/

import com.company.algorithm.AccelerationPatcher;
import com.company.algorithm.MultiThreadAlgorithm;
import com.company.algorithm.helpers.Units;
import com.company.files.BinWriter;
import com.company.files.TxtReader;
import com.sun.org.apache.bcel.internal.generic.IF_ACMPEQ;

import java.io.File;
import java.util.ArrayList;
import java.util.Scanner;

public class Menu {

    private String [] args;
    private static String filePath;
    private Scanner reader;
    private File file;
    public static String outputPath;
    private int duration;
    public static boolean iniLoaded;
    private TxtReader txtReader;

    private Thread writerThread;
    private BinWriter writer;
    private boolean threadIsOn;
    public static float avgAcceleration;



    public Menu(String [] args){
            this.args = args;
            filePath = "none";
            reader = new Scanner(System.in);
            iniLoaded = false;
            threadIsOn = false;
        }

        public void mainMenu(){

            while (true) {

                printHeader();

                System.out.println("=========");
                System.out.println("= TOOLS =");
                System.out.println("=========");
                System.out.println("");
                System.out.println("    [1] - Set initial conditions file from argument");
                System.out.println("    [2] - Set initial condition file from path");
                System.out.println("    [3] - Load initial conditions to memory");
                System.out.println("    [4] - Generate simulation (variable delta)");
                System.out.println("    [5] - Generate simulaton (fixed delta)");
                System.out.println("    [6] - Initial conditions conversion tools");
                System.out.println("    [7] - Standard deviation of acceleration (colorize by force)");
                System.out.println("    [8] - Exit");

                String command = "";
                System.out.println("");
                System.out.print("    COMMAND: ");
                command = reader.nextLine();

                if (command.trim().equals("1")){
                    setFilePathFromArg();
                }else if (command.trim().equals("2")){
                    setFilePath();
                }else if (command.trim().equals("3")){
                    printHeader();
                    if (filePath.contains(".txt")){
                        txtReader = new TxtReader(file);
                        txtReader.read();

                        System.out.println("");
                        System.out.println("");
                        System.out.println("PRESS ENTER TO CONTINUE...");
                        reader.nextLine();
                    }else {
                        System.out.println("ERROR: The selected file is either empty, or is not in .txt format");
                    }
                }else if (command.trim().equals("4")){
                    printHeader();
                    if (iniLoaded){
                        generateSimulation(false);
                    }else {
                        System.out.println("ERROR: No initial condition file has been loaded into memory");
                    }
                }else if (command.trim().equals("5")){
                    printHeader();
                    if (iniLoaded){
                        generateSimulation(true);
                    }else {
                        System.out.println("ERROR: No initial condition file has been loaded into memory");
                    }
                }else if (command.trim().equals("8")){

                    if (threadIsOn){
                        writer.terminate();

                        synchronized (writer.getLock()){
                            writerThread.interrupt();

                        }

                    }

                    break;

                }else if (command.trim().equals("7")){

                    if (threadIsOn){
                        writer.terminate();

                        synchronized (writer.getLock()){
                            writerThread.interrupt();

                        }

                        printHeader();
                        System.out.println("avg: "+avgAcceleration);
                        AccelerationPatcher patcher = new AccelerationPatcher(avgAcceleration, new File(outputPath));
                        System.out.println();
                        System.out.println("Done...");

                    }


                }



            }

        }

        public static void printHeader(){
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println();
            System.out.println("=============================================================================");
            System.out.println("=                                                                           =");
            System.out.println("=   ____       _     _ _      _____ _                 _       _             =\n" +
                    "=  / __ \\     | |   (_) |    / ____(_)               | |     | |            =\n" +
                    "= | |  | |_ __| |__  _| |_  | (___  _ _ __ ___  _   _| | __ _| |_ ___  _ __ =\n" +
                    "= | |  | | '__| '_ \\| | __|  \\___ \\| | '_ ` _ \\| | | | |/ _` | __/ _ \\| '__|=\n" +
                    "= | |__| | |  | |_) | | |_   ____) | | | | | | | |_| | | (_| | || (_) | |   =\n" +
                    "=  \\____/|_|  |_.__/|_|\\__| |_____/|_|_| |_| |_|\\__,_|_|\\__,_|\\__\\___/|_|   =");
            System.out.println("=                                                                           =");
            System.out.println("=============================================================================");
            System.out.println("                        SIMULATION GENERATION TOOL");
            System.out.println("                            By David Rockenzahn");
            System.out.println("");
            System.out.println("");
            System.out.println("    CURRENT INITIAL CONDITION FILE: [ \n    "+filePath+"\n    ]");
            System.out.println("    INITIAL CONDITIONS FILE LOADED: "+iniLoaded);
            System.out.println("");

        }

        public void setFilePathFromArg(){

            if (args.length > 0){
                File file = new File(args [0]);

                if (file.exists()){
                    filePath = args[0];
                    this.file = file;
                }
            }


        }

        public void setFilePath(){

            printHeader();
            System.out.print("    PATH AND NAME:");

            String input = reader.nextLine();

            File file = new File(input);

            if (file.exists()){
                filePath = input;
                this.file = file;
            }
        }

        private void generateSimulation(boolean fixedDelta){
                printHeader();

                System.out.println("    OUTPUT PATH: (Current by default. Enter to leave it as is.) ");
                String path = reader.nextLine();
            if (path.isEmpty()){
                path = System.getProperty("user.dir");;
                System.out.println(path);
            }
                System.out.print("    FILE NAME: (\"mySimulation\" by default. Enter to leave it as is.) ");
                String filename = reader.nextLine();
            if (filename.isEmpty()){
                filename = "mySimulation.nbd";
                System.out.println(filename);
            }else {
                filename+=".nbd";
            }

double cycles = 0;
            System.out.print("    DURATION OF SIMULATION (seconds): ");

                String input = reader.nextLine();
                if(input.isEmpty()){
                    input = "10";
                }
                float duration = Float.parseFloat(input);

                if (fixedDelta){
                    System.out.print("    CYCLES PER SECOND: ");

                    input = reader.nextLine();

                    cycles = Double.parseDouble(input);
                }

            System.out.print("    GRAVITATIONAL CONSTANT (defualt is 1): ");
                input = reader.nextLine();

                if (input.isEmpty()){
                    input="1";
                }

                float grav = Float.parseFloat(input);

            Units.GRAV = grav;

            System.out.print("    BODY SCALE (default is 1): ");
                input  =reader.nextLine();

                if (input.isEmpty()){
                    input="1";
                }

                float bodyScale = Float.parseFloat(input);

            System.out.print("    SIMULATION SPEED (default is 1): ");
            input = reader.nextLine();

            if (input.isEmpty()){
                input = "1";
            }

                float simSpeed = Float.parseFloat(input);





                File outputFile = new File(path+"/"+filename);
                outputPath = path+"/"+filename;

            System.out.println();
            System.out.print("    Sorting input data, please wait...");

            ArrayList<Float> data = txtReader.getData();

            int yot = 0;
            int s = 0;

            float m;
            int k = 0;

            float[] x;
            float[] y;
            float[] z;
            float[] vx;
            float[] vy;
            float[] vz;


            if (txtReader.getV()==1||txtReader.getV()==2){
                if (txtReader.getV()==1){
                    yot = 0;
                    s = 8;
                }else if (txtReader.getV()==2){
                    yot = 1;
                    s = 7;
                }

                x = new float[data.size()/s];
                y = new float[data.size()/s];
                z = new float[data.size()/s];
                vx = new float[data.size()/s];
                vy = new float[data.size()/s];
                vz = new float[data.size()/s];








                for (int i = 0; i<data.size(); i=i+s){

                    x[k] = data.get(i+2-yot);
                    y[k] = data.get(i+3-yot);
                    z[k] = data.get(i+4-yot);
                    vx[k] = data.get(i+5-yot);
                    vy[k] = data.get(i+6-yot);
                    vz[k] = data.get(i+7-yot);


                    k++;
                }

                m = data.get(1-yot);
            }else{
                x = new float[data.size()/11];
                y = new float[data.size()/11];
                z = new float[data.size()/11];
                vx = new float[data.size()/11];
                vy = new float[data.size()/11];
                vz = new float[data.size()/11];

                m = data.get(15);

                for (int i = 0; i<data.size(); i=i+11){

                    x[k] = data.get(i+5-yot);
                    y[k] = data.get(i+6-yot);
                    z[k] = data.get(i+7-yot);
                    vx[k] = data.get(i+8-yot);
                    vy[k] = data.get(i+9-yot);
                    vz[k] = data.get(i+10-yot);


                    k++;
                }
            }





                                                 System.out.println("           [DONE]");
            System.out.print("    Initializing accelerations, please wait...");


            data = null;

            writer = new BinWriter((short)1, x.length, bodyScale);

                MultiThreadAlgorithm multiThreadAlgorithm = new MultiThreadAlgorithm(new Object(), x,y,z,vx,vy,vz,m,simSpeed, writer, duration, fixedDelta, cycles);



            System.out.println("   [DONE]");
            System.out.print("    Initializing writer thread...");

            writerThread = new Thread(writer);
            threadIsOn = true;
            boolean v = writer.setFile(outputFile);

            if (v){
                System.out.println("                [DONE]");
            }else {
                System.out.println("                [ERROR]");
            }

            System.out.println("");

            System.out.print("PRESS ENTER TO START...");
            reader.nextLine();


            writerThread.start();
            multiThreadAlgorithm.run();



        }
}
