package com.company.screen;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 1/11/18   ==
* =====================================================================
* ==      Project: Generator tool    ==
* =====================================================================

*/

import com.company.algorithm.MultiThreadAlgorithm;
import com.company.files.BinWriter;
import com.company.files.TxtReader;

import java.io.File;
import java.util.ArrayList;
import java.util.Scanner;

public class Menu {

    private String [] args;
    private static String filePath;
    private Scanner reader;
    private File file;
    private String outputPath;
    private int duration;
    public static boolean iniLoaded;
    private TxtReader txtReader;

    private Thread writerThread;
    private BinWriter writer;
    private boolean threadIsOn;



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
                System.out.println("    [4] - Generate simulation");
                System.out.println("    [5] - Initial conditions conversion tools");
                System.out.println("    [6] - Exit");

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
                        generateSimulation();
                    }else {
                        System.out.println("ERROR: No initial condition file has been loaded into memory");
                    }
                }else if (command.trim().equals("5")){

                }else if (command.trim().equals("6")){

                    if (threadIsOn){
                        writer.terminate();
                        writerThread.interrupt();
                    }

                    break;

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

        private void generateSimulation(){
                printHeader();

                System.out.println("    OUTPUT PATH: (Current by default. Enter to leave it as is.)");
                String path = reader.nextLine();
                System.out.print("    FILE NAME: (test.txt by default. Enter to leave it as is.)");
                String filename = reader.nextLine();
                System.out.print("    DURATION (seconds): ");
                int duration = Integer.parseInt(reader.nextLine());

                if (path.isEmpty()){
                    path = System.getProperty("user.dir");;
                }

                if (filename.isEmpty()){
                    filename = "test.nbd";
                }

                File outputFile = new File(path+"/"+filename);

            System.out.println();
            System.out.print("    Sorting input data, please wait...");

            ArrayList<Float> data = txtReader.getData();

                float[] x = new float[data.size()/8];
                float[] y = new float[data.size()/8];
                float[] z = new float[data.size()/8];
                float[] vx = new float[data.size()/8];
                float[] vy = new float[data.size()/8];
                float[] vz = new float[data.size()/8];



                float m;

                int k = 0;




                for (int i = 0; i<data.size(); i=i+8){

                    x[k] = data.get(i+2);
                    y[k] = data.get(i+3);
                    z[k] = data.get(i+4);
                    vx[k] = data.get(i+5);
                    vy[k] = data.get(i+6);
                    vz[k] = data.get(i+7);


                    k++;
                }

                m = data.get(1);



            System.out.println("    [DONE]");
            System.out.print("    Initializing accelerations, please wait...");

            data = null;

            writer = new BinWriter((short)1, x.length, 1f);

            MultiThreadAlgorithm multiThreadAlgorithm = new MultiThreadAlgorithm(new Object(), x,y,z,vx,vy,vz,m,0.1f, writer, duration);

            System.out.println("    [DONE]");
            System.out.print("    Initializing writer thread...");

            writerThread = new Thread(writer);
            threadIsOn = true;
            boolean v = writer.setFile(outputFile);

            if (v){
                System.out.println("    [DONE]");
            }else {
                System.out.println("    [ERROR]");
            }

            System.out.println("");

            System.out.print("PRESS ENTER TO START...");
            reader.nextLine();


            writerThread.start();
            multiThreadAlgorithm.run();



        }
}
