package com.mygdx.game.simulation.renderer;

import com.mygdx.game.simulation.SimulationObject;
import org.joml.*;

import java.lang.Math;
import java.util.HashMap;
import java.util.Map;

/**
 * Created by Fran on 3/13/2018.
 */
public class Camera {

    protected float movementSpeed = 2.5f;
    protected float sensitivity = 0.002f;
    protected final Vector3d position;
    protected final Vector3f front, up, left, worldUp, temp, temp2;

    private float yawChange, pitchChange, rollChange;

    private int scrolledAmount;

    private final Matrix3f tempMat;

    private final AxisAngle4f tempAng;

    private SimulationObject lock = null;

    private float distFromLock;

    protected Map<Camera_Movement, Boolean> keysPressed;

    public Camera(float posX, float posY, float posZ) {
        position = new Vector3d(posX,posY,posZ);
        worldUp = new Vector3f(0, 1, 0);
        front = new Vector3f(0f, 0f, -1f);
        up = new Vector3f(0f, 1f, 0f);
        left = new Vector3f(-1f, 0f, 0f);
        temp = new Vector3f();
        temp2 = new Vector3f();
        tempMat  = new Matrix3f();
        tempAng = new AxisAngle4f();

        keysPressed = new HashMap<Camera_Movement, Boolean>();
        keysPressed.put(Camera_Movement.FORWARD, false);
        keysPressed.put(Camera_Movement.BACKWARD, false);
        keysPressed.put(Camera_Movement.RIGHT, false);
        keysPressed.put(Camera_Movement.LEFT, false);
        keysPressed.put(Camera_Movement.ROLL_LEFT,false);
        keysPressed.put(Camera_Movement.ROLL_RIGHT,false);
        keysPressed.put(Camera_Movement.UP, false);
        keysPressed.put(Camera_Movement.DOWN, false);
    }

    public Vector3d getPosition(){
        return position;
    }

    public void update(float delta){
        float velocity = movementSpeed * delta;
        for(Camera_Movement key : keysPressed.keySet()){
            if(keysPressed.get(key)){
                switch (key){
                    case FORWARD:
                        lock = null;
                        position.add(temp.set(front).mul(velocity));
                        break;
                    case BACKWARD:
                        lock = null;
                        position.add(temp.set(front).mul(-velocity));
                        break;
                    case RIGHT:
                        lock = null;
                        position.add(temp.set(left).mul(-velocity));
                        break;
                    case LEFT:
                        lock = null;
                        position.add(temp.set(left).mul(velocity));
                        break;
                    case ROLL_LEFT:
                        rollChange-=delta;
                        break;
                    case ROLL_RIGHT:
                        rollChange+=delta;
                        break;
                    case UP:
                        lock = null;
                        position.add(temp.set(up).mul(velocity));
                        break;
                    case DOWN:
                        lock = null;
                        position.add(temp.set(up).mul(-velocity));
                        break;
                }
            }
        }

        tempMat.identity().set(tempAng.set(pitchChange,left)).transform(front);
        tempMat.identity().set(tempAng.set(pitchChange,left)).transform(up);
        pitchChange = 0;

        tempMat.identity().set(tempAng.set(yawChange,up)).transform(front);
        tempMat.identity().set(tempAng.set(yawChange,up)).transform(left);
        yawChange = 0;

        tempMat.identity().set(tempAng.set(rollChange,front)).transform(up);
        tempMat.identity().set(tempAng.set(rollChange,front)).transform(left);
        rollChange = 0;

        if(lock!=null){
            temp.set(front).normalize().negate();
            if(distFromLock<lock.getRadius()){
                distFromLock=lock.getRadius();
            }
            temp.mul(lock.getRadius()+distFromLock);
            position.set(lock.getPosition()).add(temp);
        }
    }

    public void changeSpeed(int amount){
        scrolledAmount+=amount;
        if(scrolledAmount<1){
            scrolledAmount = 1;
        }
        if(lock!=null){
            distFromLock = lock.getRadius()+ (0.0794f * (float)Math.pow(1.2637,scrolledAmount));
            return;
        }

        movementSpeed = 0.0794f * (float)Math.pow(1.2637,scrolledAmount);
    }

    public void processMouseMovement(float xOffset, float yOffset){
        xOffset *= -sensitivity;
        yOffset *= -sensitivity;

        yawChange+=xOffset;
        pitchChange+=yOffset;


    }


    public void processKeyboard(Camera_Movement direction, boolean down){
        down = !down;
        keysPressed.put(direction, down);
    }

    public void setLock(SimulationObject lock) {
        this.lock = lock;
    }

    public SimulationObject getLock() {
        return lock;
    }

    public float getMovementSpeed() {
        return movementSpeed;
    }

    public Vector3f getFront() {
        return front;
    }

    public Matrix4f getViewMatrix(Matrix4f view){
        return view.identity().lookAt(temp2.set(0,0,0), front, up);
    }
}