package com.mygdx.game.playback;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 4/19/18   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/


import java.io.*;
import java.util.*;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;


public class PlayBackLoader2 implements Runnable {

    private boolean terminate;

    private AtomicBoolean loopInterrupt;

    private DataInputStream inputStream;

    private int numberOfBodies, cycles;
    private float bodyScale, minAccel, maxAccel;
    private int firstFrame, lastFrame;
    private AtomicInteger currentFrame;

    private int frameSize;
    private int listSize;

    private LinkedList<float []> frameList;

    private ListIterator<float []> frameListIterator;
    private ListIterator<float []> nextFrameForDispatch;

    private AtomicInteger loadedFrame;

    private AtomicBoolean memoryAvailable;

    private File path;

    private  int counter;

    private final Object lock;

    private CountDownLatch latch;



    public PlayBackLoader2(String path) {

        loopInterrupt=new AtomicBoolean(false);
        terminate = false;
        lastFrame=-1;
        currentFrame = new AtomicInteger(0);
        loadedFrame = new AtomicInteger(0);
        firstFrame=0;
        lock = new Object();

        latch=new CountDownLatch(1);

        try {
            this.path = new File(path);
            RandomAccessFile randomAccessFile =  new RandomAccessFile(this.path ,"r");
            randomAccessFile.seek(randomAccessFile.length()-8);

            maxAccel = randomAccessFile.readFloat();
            minAccel = randomAccessFile.readFloat();

            randomAccessFile.close();


            inputStream = new DataInputStream(new BufferedInputStream(new FileInputStream(this.path)));
            long length = inputStream.available() - 18;
            short version = inputStream.readShort();


            if (version == 1) {
                numberOfBodies = inputStream.readInt();
                bodyScale = inputStream.readFloat();
                cycles = (int) length / (16 * numberOfBodies);
            }

            long availableMemory = Runtime.getRuntime().maxMemory()-Runtime.getRuntime().freeMemory();
            System.out.println("Max memory: "+Runtime.getRuntime().maxMemory()/1000000D+" mega bytes");
            System.out.println("Total memory: "+Runtime.getRuntime().freeMemory()/1000000D+" mega bytes");
            frameSize = 4*numberOfBodies;
            System.out.println("Available mem.: "+availableMemory/(1000000D)+" mega bytes");
            listSize = (int) (availableMemory*(0.9D) / (16D*numberOfBodies));
            System.out.println("List memory: "+((listSize*4D*frameSize)/1000000D)+" mega bytes");

            frameList = new LinkedList<float[]>();

            for (int i = 0; i < listSize; i++){
                frameList.add(new float [frameSize]);
            }

            frameListIterator = frameList.listIterator();
            nextFrameForDispatch = frameList.listIterator();



        } catch (Exception e) {
            e.printStackTrace();
        }

    }

    @Override
    public void run() {
        try {
            long bytesPerFrame = 16*numberOfBodies;

            float [] tmpFrame;

            memoryAvailable = new AtomicBoolean(true);

            System.out.println("listSize: "+listSize+" frames");

            counter =0;

            while (!terminate){

                if (loopInterrupt.get()){
                    loopInterrupt.set(false);
                    latch.await();
                }

                if (memoryAvailable.get()){

                    synchronized (lock) {

                        if (counter>= listSize-1){
                            memoryAvailable.set(false);
                            frameListIterator = frameList.listIterator();
                            counter=0;
                        }else {
                            if (inputStream.available() >= frameSize * 4) {
                                tmpFrame = frameListIterator.next();
                                loadedFrame.getAndIncrement();
                                //System.out.println(loadedFrame);
                                for (int k = 0; k < frameSize; k++) {
                                    tmpFrame[k] = inputStream.readFloat();
                                }
                            }
                            counter++;
                        }
                            }

                }else {
                    //if (loadedFrame%frameSize < currentFrame.get()-1 && inputStream.available()>=frameSize*4){
                    if (firstFrame < currentFrame.get()-1 && inputStream.available()>=frameSize*4){

                        if ((loadedFrame.get()+1)%listSize==0){
                            frameListIterator = frameList.listIterator();
                        }

                        synchronized (lock){
                            tmpFrame = frameListIterator.next();
                            loadedFrame.getAndIncrement();
                            firstFrame++;
                            for (int k = 0; k < frameSize; k++){
                                tmpFrame [k] = inputStream.readFloat();
                            }
                        }

                    }


                }
            }
        }catch (Exception e){
            e.printStackTrace();
            System.out.println("Frame size: "+frameSize);
        }

    }

    public float [] requestNextFrame(){

        if (loadedFrame.get() <= currentFrame.get()+1){
            return null;
        }else {
            //System.out.println("c: "+currentFrame);
            currentFrame.incrementAndGet();
            if (nextFrameForDispatch.hasNext()){
                return nextFrameForDispatch.next();
            }else {
                nextFrameForDispatch = frameList.listIterator();
                return nextFrameForDispatch.next();
            }
        }


    }

    public boolean containsFrame(int frame){
        return (frame<=loadedFrame.get());
    }

    public void skipFrames(int amountOfFrames){

        //synchronized (lock){
            for (int i = 0; i < amountOfFrames; i++){
                currentFrame.getAndIncrement();
                if (nextFrameForDispatch.hasNext()){
                    nextFrameForDispatch.next();
                }else {
                    nextFrameForDispatch = frameList.listIterator();
                    //nextFrameForDispatch.next();
                }
            }
        //}
    }

    public void reverseFrames(int amountOfFrames){
        for (int i = 0; i < amountOfFrames; i++){
            currentFrame.getAndDecrement();
            if (nextFrameForDispatch.hasPrevious()){
                nextFrameForDispatch.previous();
            }else {
                nextFrameForDispatch = frameList.listIterator(listSize-1);
                //nextFrameForDispatch.previous();
            }
        }
    }

    public void changeFramePosition(int newFrame){

        long delta = System.nanoTime();
        if (newFrame <= loadedFrame.get()&&newFrame>=currentFrame.get()){
            skipFrames(newFrame-currentFrame.get());
        }else  if (newFrame <=currentFrame.get()&&newFrame>=firstFrame){
            reverseFrames(currentFrame.get()-newFrame);
        }else {
            firstFrame=newFrame;
            synchronized (lock){
                latch=new CountDownLatch(1);
                loopInterrupt.set(true);
                memoryAvailable.set(true);
                currentFrame.getAndSet(newFrame);
                frameListIterator = frameList.listIterator(0);
                nextFrameForDispatch = frameList.listIterator(0);
                loadedFrame.getAndSet(newFrame);
                counter=0;


                try {
                    inputStream.close();
                    inputStream = new DataInputStream(new BufferedInputStream(new FileInputStream(this.path)));
                    inputStream.skipBytes(10+(frameSize*4*newFrame));
                }catch (Exception e){
                    e.printStackTrace();
                }
            }
            latch.countDown();
        }
        System.out.println("Delta: "+(System.nanoTime()-delta)/1000000000D);
    }

    /*
    public Vector<Pair<Vector3,Float>> requestFrame(int frame){

        currentFrame=frame;
        if (frame < firstFrame || frame > lastFrame) {

         //   frameMap = new ConcurrentHashMap<Integer, Vector<Pair<Vector3, Float>>>();
            firstFrame=frame;
            lastFrame=frame;
            System.gc();
        }

      //  if (frameMap.containsKey(frame)) {
       //     return frameMap.get(frame);
      //  } else {
       //     return null;
       // }

    }
    */

    public void terminate(){
        terminate = true;
    }

    public int getNumberOfBodies() {
        return numberOfBodies;
    }

    public float getBodyScale() {
        return bodyScale;
    }

    public float getMinAccel() {
        return minAccel;
    }

    public float getMaxAccel() {
        return maxAccel;
    }

    public int getCycles() {
        return cycles;
    }

  //  public ConcurrentHashMap<Integer, Vector<Pair<Vector3, Float>>> getFrameMap() {
    //    return frameMap;
   // }

    public int getLastFrame() {
        return loadedFrame.get();
    }

    public int getFirstFrame() {
        return firstFrame;
    }
}
