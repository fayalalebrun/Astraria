package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.utils.Disposable;
import com.mokiat.data.front.parser.*;
import org.joml.Matrix4f;
import org.joml.Vector3f;
import org.joml.Vector3i;

import java.io.*;
import java.net.URI;
import java.net.URL;
import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.*;

/**
 * Created by fraayala19 on 3/15/18.
 */
public class Model implements Disposable{

    protected ArrayList<Mesh> meshes;
    private final Vector3f position, rotation;
    private final Transformation transformation;
    private float scale;

    private final Matrix4f rotationMat;

    private boolean useEulerRot = false;

    private float z;

    public Model(ArrayList<Mesh> meshes, Transformation transformation){
        position = new Vector3f();
        scale = 1f;
        rotation = new Vector3f();
        rotationMat = new Matrix4f().identity();
        this.meshes = meshes;
        this.transformation = transformation;
    }


    private String getPath(String name){
        return Gdx.files.local("/models/"+name).file().getAbsolutePath();
    }

    public void setPosition(float x, float y, float z){
        position.set(x,y,z);
    }

    public void setPosition(Vector3f pos){
        position.set(pos);
    }

    public void setRotation(float x, float y, float z){
        rotation.set(x, y, z);
        useEulerRot = true;
    }

    public void render(Camera cam, Shader shader){
        shader.use();
        if(useEulerRot) {
            shader.setMat4("modelView", transformation.getModelViewMatrix(transformation.getViewMatrix(cam), position, rotation, scale));
        } else {
            shader.setMat4("modelView", transformation.getModelViewMatrix(transformation.getViewMatrix(cam), position, rotationMat, scale));
        }
        for (Mesh mesh : this.meshes){
            mesh.render(shader);
        }
    }

    @Override
    public void dispose() {
        for(Mesh mesh : meshes){
            mesh.dispose();
        }
    }

    public void setScale(float scale) {
        this.scale = scale;
    }

    public void setRotation(Matrix4f rotationMat){
        this.rotationMat.set(rotationMat);
        useEulerRot = false;
    }

    public ArrayList<Mesh> getMeshes() {
        return meshes;
    }
}
