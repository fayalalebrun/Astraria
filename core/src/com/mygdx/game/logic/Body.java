package com.mygdx.game.logic;

/**
 * Created by fraayala19 on 12/12/17.
 */
public class Body {
    protected double mass;
    protected double x, y, z;
    protected double vX, vY, vZ;


    public Body(double mass, double x, double y, double z, double vX, double vY, double vZ) {
        this.mass = mass;
        this.x = x;
        this.y = y;
        this.z = z;
        this.vX = vX;
        this.vY = vY;
        this.vZ = vZ;
    }

    public double getMass() {
        return mass;
    }

    public void setMass(double mass) {
        this.mass = mass;
    }

    public double getX() {
        return x;
    }

    public void setX(double x) {
        this.x = x;
    }

    public double getY() {
        return y;
    }

    public void setY(double y) {
        this.y = y;
    }

    public double getZ() {
        return z;
    }

    public void setZ(double z) {
        this.z = z;
    }

    public double getvX() {
        return vX;
    }

    public void setvX(double vX) {
        this.vX = vX;
    }

    public double getvY() {
        return vY;
    }

    public void setvY(double vY) {
        this.vY = vY;
    }

    public double getvZ() {
        return vZ;
    }

    public void setvZ(double vZ) {
        this.vZ = vZ;
    }
}
