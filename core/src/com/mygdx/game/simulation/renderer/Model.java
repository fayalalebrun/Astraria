package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import de.javagl.obj.*;
import org.joml.Matrix4f;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;

/**
 * Created by fraayala19 on 3/15/18.
 */
public class Model {

    OpenGLTextureManager textures;
    ArrayList<Mesh> meshes;
    Shader shader;

    public Model(OpenGLTextureManager textureManager, String name, Shader shader) {
        this.textures = textureManager;
        meshes = new ArrayList<Mesh>();
        this.shader = shader;
        String path = getPath(name);
        loadModel(path);
    }

    private void loadModel(String path){
        try {
            InputStream objInputStream = new FileInputStream(path);
            Obj originalObj = ObjReader.read(objInputStream);

            Obj obj = ObjUtils.convertToRenderable(originalObj);

            List<Mtl> allMtls = new ArrayList<Mtl>();
            for(String mtlFileName : obj.getMtlFileNames()){
                InputStream mtlInputStream = new FileInputStream(getPath(mtlFileName));
                List<Mtl> mtls = MtlReader.read(mtlInputStream);
                allMtls.addAll(mtls);
            }

            Map<String, Obj> materialGroups = ObjSplitting.splitByMaterialGroups(obj);

            for(Map.Entry<String, Obj> entry : materialGroups.entrySet()){
                String materialName = entry.getKey();
                Obj materialGroup = entry.getValue();

                Mtl mtl = findMtlForName(allMtls, materialName);

                processMesh(mtl, materialGroup);
            }
            
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private void processMesh(Mtl mtl, Obj obj){
        String diffuseTex = mtl.getMapKd();

        if(diffuseTex.startsWith("d ")) {
            diffuseTex = diffuseTex.substring(2);
        }

        int[] indices = ObjData.getFaceVertexIndicesArray(obj, 3);
        float[] vertices = ObjData.getVerticesArray(obj);
        float[] texCoords = ObjData.getTexCoordsArray(obj, 2);
        float[] normals = ObjData.getNormalsArray(obj);




        meshes.add(new Mesh(indices,vertices,texCoords,normals,textures.addTexture(getPath(diffuseTex))));
    }

    private String getPath(String name){
        return Gdx.files.local("/models/"+name).file().getAbsolutePath();
    }

    private Mtl findMtlForName(Iterable<? extends Mtl> mtls, String name)
    {
        for (Mtl mtl : mtls)
        {
            if (mtl.getName().equals(name))
            {
                return mtl;
            }
        }
        return null;
    }

    public void render(Matrix4f modelMatrix){
        shader.use();
        shader.setMat4("model", modelMatrix);
        for (Mesh mesh : this.meshes){
            mesh.render(shader);
        }
    }

}
