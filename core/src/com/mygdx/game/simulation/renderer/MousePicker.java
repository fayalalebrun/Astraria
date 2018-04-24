package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector3f;
import org.joml.Vector4f;

import java.util.Vector;

/**
 * Created by Fran on 4/22/2018.
 */
public class MousePicker {

    private Vector3f currentRay;

    private final Matrix4f projectionMatrix;
    private final Matrix4f viewMatrix;

    private final Vector4f clipCoords = new Vector4f();
    private final Vector3f mouseRay = new Vector3f();
    private final Vector4f eyeCoords = new Vector4f();
    private final Vector2f deviceCoords = new Vector2f();

    public MousePicker() {
        projectionMatrix = new Matrix4f();
        viewMatrix = new Matrix4f();
    }

    public Vector3f getCurrentRay(){
        return currentRay;
    }

    public void update(Matrix4f viewMatrix, Matrix4f projectionMatrix, int screenWidth, int screenHeight){
        this.viewMatrix.set(viewMatrix);
        this.projectionMatrix.set(projectionMatrix);
        currentRay = calculateMouseRay(screenWidth, screenHeight);
    }

    public Vector3f calculateMouseRay(int screenWidth, int screenHeight){
        float mouseX = Gdx.input.getX();
        float mouseY = Gdx.input.getY();
        Vector2f normalizedCoords = getNormalizedDeviceCoords(mouseX, mouseY, screenWidth, screenHeight);
        Vector4f clipCoords =  this.clipCoords.set(normalizedCoords.x, normalizedCoords.y, -1f, 1f);
        Vector4f eyeCoords = toEyeCoords(clipCoords);
        Vector3f worldRay = toWorldCoords(eyeCoords);
        return worldRay;
    }

    private Vector3f toWorldCoords(Vector4f eyeCoords){
        viewMatrix.invert();
        Vector4f rayWorld = viewMatrix.transform(eyeCoords);
        Vector3f mouseRay = this.mouseRay.set(rayWorld.x, rayWorld.y, rayWorld.z);
        mouseRay.normalize();
        return mouseRay;
    }

    private Vector4f toEyeCoords(Vector4f clipCoords){
        projectionMatrix.invert();
        Vector4f eyeCoords = projectionMatrix.transform(clipCoords);
        return this.eyeCoords.set(eyeCoords.x, eyeCoords.y, -1f, 0);
    }

    private Vector2f getNormalizedDeviceCoords(float mouseX, float mouseY, int screenWidth, int screenHeight){
        float x = (2f*mouseX) / screenWidth - 1f;
        float y = (2f*mouseY) / screenHeight - 1f;
        return deviceCoords.set(x,-y);
    }
}
