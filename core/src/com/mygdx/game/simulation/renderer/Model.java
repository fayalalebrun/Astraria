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

    private ArrayList<Mesh> meshes;
    private Shader shader;
    private final Vector3f position, rotation;
    private final Transformation transformation;
    private float scale;



    public Model(ArrayList<Mesh> meshes, Shader shader, Transformation transformation){
        position = new Vector3f();
        scale = 1f;
        rotation = new Vector3f();
        this.meshes = meshes;
        this.shader = shader;
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
    }

    public void render(Camera cam){
        shader.use();
        shader.setMat4("modelView", transformation.getModelViewMatrix(transformation.getViewMatrix(cam),position,rotation, scale));
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
}
