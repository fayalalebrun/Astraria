package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.utils.Disposable;
import org.joml.Vector3f;

/**
 * Created by Fran on 4/10/2018.
 */
public class Sphere implements Disposable {
    private Mesh sphereMesh;
    private final Vector3f position, rotation;
    private final Transformation transformation;
    private float scale;

    public Sphere(ModelManager modelManager, Transformation transformation){
        position = new Vector3f();
        this.transformation = transformation;
        rotation = new Vector3f();
        this.sphereMesh = modelManager.loadModel("sphere.obj", transformation).getMeshes().get(0);
        scale = 1f;
    }
    public void setPosition(float x, float y, float z){
        position.set(x,y,z);
    }

    public void setPosition(Vector3f pos){
        position.set(pos);
    }

    public void setRotation(float x, float y, float z){
        rotation.set(x, y, z);
    }

    public void render(Camera cam, Shader shader){
        shader.use();
        shader.setMat4("modelView", transformation.getModelViewMatrix(transformation.getViewMatrix(cam),position,rotation, scale));
        sphereMesh.render(shader);
    }

    @Override
    public void dispose() {
        sphereMesh.dispose();
    }


}
