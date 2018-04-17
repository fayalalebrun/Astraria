package com.mygdx.game.simulation;

import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.utils.Disposable;
import com.mygdx.game.simulation.logic.Body;
import com.mygdx.game.simulation.renderer.Camera;
import com.mygdx.game.simulation.renderer.Model;
import com.mygdx.game.simulation.renderer.Shader;
import com.mygdx.game.simulation.renderer.Transformation;
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
    private final String name;
    protected final Shader shader;
    protected final Transformation transformation;
    protected final Body body;
    protected final Color orbitColor;
    protected final Orbit orbit;
    protected final Clickable3DObject clickable3DObject;

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

        setRotationParameters((float)Math.PI/2f,0f,1f,0f);
    }

    public void setRotationParameters(float inclinationTilt, float axisRightAscension, float rotationPeriod, float offset){
        this.inclinationTilt = inclinationTilt;
        this.axisRightAscension = axisRightAscension;
        this.rotationPeriod = rotationPeriod;
        rotationSpeed = 1/rotationPeriod;

        this.rotationAxis.set(Transformation.WORLD_UP);
        rotationAccum.identity().rotate(tempAng.set(inclinationTilt,transformation.WORLD_RIGHT)).translate(rotationAxis);
        rotationAccum.rotate(tempAng.set(axisRightAscension,transformation.WORLD_UP)).translate(rotationAxis);

        rotationAccum.rotate(tempAng.set(offset,rotationAxis));
    }

    public void updatePosition(){
        synchronized (body){
            this.position.set(body.getX()/1000, body.getZ()/1000, body.getY()/1000);

        }
    }

    protected void update(Camera cam, float delta){
        if(rotationSpeed>0){
            rotationAccum.rotate(tempAng.set(rotationSpeed*delta*SimulationScreen.simSpeed,rotationAxis));
        }

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

    public Clickable3DObject getClickable3DObject() {
        return clickable3DObject;
    }
}
