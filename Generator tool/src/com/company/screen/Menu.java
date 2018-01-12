package com.company.screen;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 1/11/18   ==
* =====================================================================
* ==      Project: Generator tool    ==
* =====================================================================

*/

import java.io.File;
import java.util.Scanner;

public class Menu {

    private String [] args;
    private String filePath;
    private Scanner reader;
    private File file;
    private String outputPath;
    private int duration;
    public static boolean iniLoaded;



        public Menu(String [] args){
            this.args = args;
            filePath = "none";
            reader = new Scanner(System.in);
            iniLoaded = false;

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
                        com.company.files.FileReader fileReader = new com.company.files.FileReader(file);
                        fileReader.read();

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
                    break;

                }


            }

        }

        public void printHeader(){
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
                    path = "none";
                }

                if (filename.isEmpty()){
                    filename = "test.txt";
                }



        }
}
