package com.mygdx.game.simulation.logic;

/**
 * Created by fraayala19 on 12/12/17.
 */
public class Body {
    protected double mass;
    protected double x, y, z;
    protected double vX, vY, vZ, currAccelX=Double.NaN, currAccelY = Double.NaN, currAccelZ = Double.NaN;
    private boolean accelInit = false;


    public Body(double mass, double x, double y, double z, double vX, double vY, double vZ) {
        this.mass = mass; //kilograms
        this.x = x; //meters
        this.y = y; //meters
        this.z = z; //meters
        this.vX = vX; //meters per second
        this.vY = vY; //meters per second
        this.vZ = vZ; //meters per second
    }

    public double getMass() {
        return mass;
    }

    public synchronized void setMass(double mass) {
        this.mass = mass;
    }

    public double getX() {
        return x;
    }

    public synchronized void setX(double x) {
        this.x = x;
    }

    public double getY() {
        return y;
    }

    public synchronized void setY(double y) {
        this.y = y;
    }

    public double getZ() {
        return z;
    }

    public synchronized void setZ(double z) {
        this.z = z;
    }

    public double getvX() {
        return vX;
    }

    public synchronized void setvX(double vX) {
        this.vX = vX;
    }

    public double getvY() {
        return vY;
    }

    public synchronized void setvY(double vY) {
        this.vY = vY;
    }

    public double getvZ() {
        return vZ;
    }

    public synchronized void setvZ(double vZ) {
        this.vZ = vZ;
    }

    public boolean getAccelInit(){
        return accelInit;
    }

    public synchronized void setAccelInitTrue(){
        accelInit = true;
    }

    public double getCurrAccelX() {
        return currAccelX;
    }

    public synchronized void setCurrAccelX(double currAccelX) {
        this.currAccelX = currAccelX;
    }

    public double getCurrAccelY() {
        return currAccelY;
    }

    public synchronized void setCurrAccelY(double currAccelY) {
        this.currAccelY = currAccelY;
    }

    public double getCurrAccelZ() {
        return currAccelZ;
    }

    public synchronized void setCurrAccelZ(double currAccelZ) {
        this.currAccelZ = currAccelZ;
    }
}
