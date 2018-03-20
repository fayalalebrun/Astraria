package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.math.Matrix3;
import com.sun.org.apache.xpath.internal.operations.Bool;
import org.joml.Matrix3f;
import org.joml.Matrix4f;
import org.joml.Vector3d;
import org.joml.Vector3f;

import java.util.HashMap;
import java.util.Map;

/**
 * Created by Fran on 3/13/2018.
 */
public class Camera {

    private float movementSpeed = 2.5f;
    private float sensitivity = 0.2f;
    private final Vector3d position;
    private final Vector3f front, up, right, worldUp, temp, temp2;

    private float yaw, pitch;

    private int scrolledAmount;

    Map<Camera_Movement, Boolean> keysPressed;

    public Camera(float posX, float posY, float posZ) {
        position = new Vector3d(posX,posY,posZ);
        worldUp = new Vector3f(0, 1, 0);
        front = new Vector3f();
        up = new Vector3f();
        right = new Vector3f();
        temp = new Vector3f();
        temp2 = new Vector3f();
        yaw = -90f;
        pitch = 0f;
        updateCameraVectors();

        keysPressed = new HashMap<Camera_Movement, Boolean>();
        keysPressed.put(Camera_Movement.FORWARD, false);
        keysPressed.put(Camera_Movement.BACKWARD, false);
        keysPressed.put(Camera_Movement.RIGHT, false);
        keysPressed.put(Camera_Movement.LEFT, false);
    }

    private void updateCameraVectors() {
        front.x = (float)(Math.cos(Math.toRadians(yaw))*Math.cos(Math.toRadians(pitch)));
        front.y = (float)Math.sin(Math.toRadians(pitch));
        front.z = (float)(Math.sin(Math.toRadians(yaw)) * Math.cos(Math.toRadians(pitch)));
        front.normalize();

        right.set(front).cross(worldUp).normalize();

        up.set(right).cross(front).normalize();

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
                        position.add(temp.set(front).mul(velocity));
                        break;
                    case BACKWARD:
                        position.add(temp.set(front).mul(-velocity));
                        break;
                    case RIGHT:
                        position.add(temp.set(right).mul(velocity));
                        break;
                    case LEFT:
                        position.add(temp.set(right).mul(-velocity));
                        break;
                }
            }
        }
    }

    public void changeSpeed(int amount){
        scrolledAmount+=amount;
        if(scrolledAmount<1){
            scrolledAmount = 1;
        }
        System.out.println(scrolledAmount);
        movementSpeed = (float)2.5 * (float)Math.pow((double)scrolledAmount, 4);
        System.out.println(movementSpeed);
    }

    public void processMouseMovement(float xOffset, float yOffset){
        xOffset *= sensitivity;
        yOffset *= sensitivity;

        yaw += xOffset;
        pitch += yOffset;

        if(pitch > 89f){
            pitch = 89f;
        }
        if (pitch < -89f){
            pitch = -89f;
        }

        updateCameraVectors();
    }


    public void processKeyboard(Camera_Movement direction, boolean down){
        down = !down;
        keysPressed.put(direction, down);
    }

    public Vector3f getFront() {
        return front;
    }

    public Matrix4f getViewMatrix(Matrix4f view){
        return view.identity().lookAt(temp2.set(0,0,0), front, up);
    }
}