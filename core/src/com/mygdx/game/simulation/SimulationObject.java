package com.mygdx.game.simulation;

import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.utils.Disposable;
import com.mygdx.game.simulation.logic.Body;
import com.mygdx.game.simulation.renderer.Camera;
import com.mygdx.game.simulation.renderer.Model;
import com.mygdx.game.simulation.renderer.Shader;
import com.mygdx.game.simulation.renderer.Transformation;
import com.mygdx.game.simulation.ui.stat_trackers.SimObjectTracker;
import org.joml.AxisAngle4f;
import org.joml.Matrix4f;
import org.joml.Vector3d;
import org.joml.Vector3f;

/**
 * Created by Fran on 3/17/2018.
 */
public class SimulationObject implements Disposable{
    protected final Vector3d position, temp;
    private final Vector3f temp2;
    private final Model model;
    private float radius;
    private String name;
    protected final Shader shader;
    protected final Transformation transformation;
    protected final Body body;
    protected final Color orbitColor;
    protected final Orbit orbit;
    protected final Clickable3DObject clickable3DObject;

    protected SimObjectTracker tracker;

    private float inclinationTilt, axisRightAscension, rotationPeriod, rotationSpeed;
    private final Vector3f rotationAxis;
    private final Matrix4f rotationAccum;

    private final AxisAngle4f tempAng;

    public SimulationObject(Model model, Shader shader, Shader orbitShader, float radius, String name, Transformation transformation, Body body, Color orbitColor) {
        position = new Vector3d();
        this.model = model;
        this.temp = new Vector3d();
        this.temp2 = new Vector3f();
        rotationAxis = new Vector3f().set(0f,1f,0f);
        rotationAccum = new Matrix4f();
        tempAng = new AxisAngle4f();
        this.radius = radius; //km
        this.name = name;
        this.shader = shader;
        this.transformation = transformation;
        this.body = body;
        this.orbitColor = orbitColor;
        orbit = new Orbit(orbitColor,5000000f, orbitShader);
        clickable3DObject = new Clickable3DObject(this);

        createTracker();
    }

    protected void createTracker(){
        tracker = new SimObjectTracker(this);
    }

    public void setRotationParameters(float inclinationTilt, float axisRightAscension, float rotationPeriod, float offset){
        this.inclinationTilt = (float)Math.toRadians(inclinationTilt);
        this.axisRightAscension = (float)Math.toRadians(axisRightAscension);
        this.rotationPeriod = (float)Math.toRadians(rotationPeriod);
        rotationSpeed = rotationPeriod/86400f;

        this.rotationAxis.set(Transformation.WORLD_UP);
        rotationAccum.identity().rotate(tempAng.set(this.inclinationTilt,transformation.WORLD_RIGHT)).translate(rotationAxis);
        rotationAccum.rotate(tempAng.set(this.axisRightAscension,transformation.WORLD_UP)).translate(rotationAxis);

        rotationAccum.rotate(tempAng.set((float)Math.toRadians(offset),rotationAxis));
    }

    public void updatePosition(){
        synchronized (body){
            this.position.set(body.getX()/1000, body.getZ()/1000, body.getY()/1000);

        }
    }

    protected void update(Camera cam, float delta){
        rotationAccum.rotate(tempAng.set(rotationSpeed*delta*SimulationScreen.simSpeed,rotationAxis));


        orbit.prepare(position.x, position.y, position.z,cam);
        model.setScale(radius);
        model.setPosition(getPositionRelativeToCamera(cam));
        model.setRotation(rotationAccum);
    }

    public void render(Camera cam, float delta){
        update(cam, delta);
        model.render(cam, shader);
    }


    public Vector3f getPositionRelativeToCamera(Camera cam){
        return temp2.set(temp.set(position).sub(cam.getPosition()));
    }

    public String getName() {
        return name;
    }

    public void setName(String name){
        this.name = name;
    }

    public float getRadius() {
        return radius;
    }

    @Override
    public void dispose() {

    }

    public Vector3d getPosition() {
        return position;
    }

    public Color getOrbitColor() {
        return orbitColor;
    }

    public Orbit getOrbit() {
        return orbit;
    }

    public Body getBody() {
        return body;
    }

    public void setRadius(float radius) {
        this.radius = radius;
    }

    public Clickable3DObject getClickable3DObject() {
        return clickable3DObject;
    }

    public SimObjectTracker getTracker() {
        return tracker;
    }

    public void setMass(double mass){
        synchronized (body){
            body.setMass(mass);
        }
    }

    public double getMass(){
        return body.getMass();
    }

    public void setXPos(double xPos){
        synchronized (body){
            body.setX(xPos);
        }
    }

    public double getXPos(){
        return body.getX();
    }

    public void setYPos(double yPos){
        synchronized (body){
            body.setY(yPos);
        }
    }

    public double getYPos(){
        return body.getY();
    }

    public void setZPos(double zPos){
        synchronized (body){
            body.setZ(zPos);
        }
    }

    public double getZPos(){
        return body.getZ();
    }

    public double getXVel(){
        return body.getvX();
    }

    public void setXVel(double xVel){
        synchronized (body){
            body.setX(xVel);
        }
    }

    public double getYVel(){
        return body.getvY();
    }

    public void setYVel(double yVel){
        synchronized (body){
            body.setvY(yVel);
        }
    }

    public double getZVel(){
        return body.getvZ();
    }

    public void setZVel(double zVel){
        synchronized (body){
            body.setvZ(zVel);
        }
    }

    public float getInclination(){
        return inclinationTilt;
    }

    public void setInclination(float inclination){
        setRotationParameters(inclination,axisRightAscension,rotationPeriod,0);
    }

    public float getAxisRightAscension(){
        return axisRightAscension;
    }

    public void setAxisRightAscension(float axisRightAscension){
        setRotationParameters(inclinationTilt,axisRightAscension,rotationPeriod,0);
    }

    public float getRotationPeriod(){
        return rotationPeriod;
    }

    public void setRotationPeriod(float rotationPeriod){
        setRotationParameters(inclinationTilt,axisRightAscension,rotationPeriod,0);
    }
}
