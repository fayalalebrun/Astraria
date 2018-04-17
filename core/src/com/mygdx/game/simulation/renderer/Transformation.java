package com.mygdx.game.simulation.renderer;

import org.joml.Matrix4d;
import org.joml.Matrix4f;
import org.joml.Vector3f;

/**
 * Created by Fran on 3/14/2018.
 */
public class Transformation {
    private final Matrix4f projectionMatrix;

    private final Matrix4f worldMatrix;

    private final Matrix4f viewMatrix;

    private final Matrix4f modelViewMatrix;

    private final Matrix4f viewCurr;

    public Transformation(){
        projectionMatrix = new Matrix4f();

        worldMatrix = new Matrix4f();

        viewMatrix = new Matrix4f();

        modelViewMatrix = new Matrix4f();

        viewCurr = new Matrix4f();
    }

    public final Matrix4f getProjectionMatrix(float fov, float width, float height, float zNear, float zFar){
        float aspectRatio = width / height;
        projectionMatrix.identity();
        projectionMatrix.perspective(fov, aspectRatio, zNear, zFar);
        return projectionMatrix;
    }

    public Matrix4f getWorldMatrix(Vector3f offset, Vector3f rotation, float scale){
        worldMatrix.identity().translate(offset).
                rotateX((float)Math.toRadians(rotation.x)).
                rotateY((float)Math.toRadians(rotation.y)).
                rotateZ((float)Math.toRadians(rotation.z)).
                scale(scale);
        return worldMatrix;
    }

    public Matrix4f getModelViewMatrix(Matrix4f viewMatrix, Vector3f offset, Vector3f rotation, float scale) {
        modelViewMatrix.identity().translate(offset).
                rotateX((float)Math.toRadians(-rotation.x)).
                rotateY((float)Math.toRadians(-rotation.y)).
                rotateZ((float)Math.toRadians(-rotation.z)).
                scale(scale);

        viewCurr.set(viewMatrix);
        return viewCurr.mul(modelViewMatrix);
    }

    public Matrix4f getModelViewMatrix(Matrix4f viewMatrix, Vector3f offset, Matrix4f rotation, float scale ){
        modelViewMatrix.identity().translate(offset).mul(rotation).scale(scale);

        viewCurr.set(viewMatrix);
        return viewCurr.mul(modelViewMatrix);
    }

    public Matrix4f getViewMatrix(Camera camera) {

        return camera.getViewMatrix(viewMatrix);
    }

}
