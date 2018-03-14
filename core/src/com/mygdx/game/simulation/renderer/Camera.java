package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.math.Matrix3;
import org.joml.Matrix3f;
import org.joml.Matrix4f;
import org.joml.Vector3f;

/**
 * Created by Fran on 3/13/2018.
 */
public class Camera {
    Vector3f position;
    Vector3f front;
    Vector3f up;
    Vector3f right;
    Vector3f worldUp;

    float yaw;
    float pitch;

    float movementSpeed;
    float mouseSensitivity;
    float zoom;

    public Camera(Vector3f position, Vector3f up, float yaw, float pitch, float movementSpeed, float mouseSensitivity, float zoom) {
        this.position = position;
        this.up = up;
        this.yaw = yaw;
        this.pitch = pitch;

        this.movementSpeed = movementSpeed;
        this.mouseSensitivity = mouseSensitivity;
        this.zoom = zoom;

        updateCameraVectors();
    }

    public Matrix4f getViewMatrix(){
        return (new Matrix4f()).lookAt(position, (new Vector3f(position)).add(front),up);
    }

    public void processKeyboard(Camera_Movement direction, float delta){
        float velocity = movementSpeed * delta;
        switch (direction){
            case FORWARD:
                position.add(new Vector3f(front).mul(velocity));
                break;
            case BACKWARD:
                position.sub(new Vector3f(front).mul(velocity));
                break;
            case LEFT:
                position.sub(new Vector3f(right).mul(velocity));
                break;
            case RIGHT:
                position.sub(new Vector3f(right).mul(velocity));
                break;
        }
    }

    private void updateCameraVectors(){
        Vector3f front = new Vector3f();
        front.x = (float)(Math.cos(Math.toRadians(yaw)) * Math.cos(Math.toRadians(pitch)));
        front.y = (float)Math.sin(Math.toRadians(pitch));
        front.z = (float)(Math.sin(Math.toRadians(yaw))*Math.cos(Math.toRadians(pitch)));
        this.front = front.normalize();
        right = (new Vector3f(this.front)).cross(worldUp).normalize();
        up = (new Vector3f(right)).cross(this.front).normalize();
    }
}
