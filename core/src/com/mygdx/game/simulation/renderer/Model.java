package com.mygdx.game.simulation.renderer;

import de.javagl.obj.Obj;
import de.javagl.obj.ObjReader;
import de.javagl.obj.ObjUtils;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.util.ArrayList;

/**
 * Created by fraayala19 on 3/15/18.
 */
public class Model {
    OpenGLTextureManager textures;
    ArrayList<Mesh> meshes;

    public Model(OpenGLTextureManager textureManager, String name) {
        this.textures = textureManager;
        meshes = new ArrayList<Mesh>();
        String path = "./"+name;
        loadModel(path);
    }

    private void loadModel(String path){
        try {
            InputStream objInputStream = new FileInputStream(path);
            Obj originalObj = ObjReader.read(objInputStream);

            Obj obj = ObjUtils.convertToRenderable(originalObj);

            
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }


}
