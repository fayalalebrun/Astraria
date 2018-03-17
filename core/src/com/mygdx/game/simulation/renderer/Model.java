package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.mokiat.data.front.parser.*;
import org.joml.Matrix4f;
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
        try{
            InputStream in = new FileInputStream(path);
            final IOBJParser parser = new OBJParser();
            final OBJModel model = parser.parse(in);
            final IMTLParser mtlParser = new MTLParser();
            Map<String, MTLMaterial> materials = new HashMap<String, MTLMaterial>();

            for(String libraryReference:model.getMaterialLibraries()){
                final InputStream mtlStream = new FileInputStream(getPath(libraryReference));
                final MTLLibrary library = mtlParser.parse(mtlStream);

                for(MTLMaterial material : library.getMaterials()){
                    materials.put(material.getName(), material);
                }
            }


            for(OBJObject object : model.getObjects()){
                for (OBJMesh mesh : object.getMeshes()){
                    Map<OBJDataReference,Integer> usedRefs = new HashMap<OBJDataReference, Integer>();
                    List<Integer> indices = new ArrayList<Integer>();
                    List<Float> vertices = new ArrayList<Float>();
                    List<Float> texCoords = new ArrayList<Float>();
                    List<Float> normals = new ArrayList<Float>();
                    int indiceNo = 0;
                    int diffuseTexture = -1;

                    for(OBJFace face : mesh.getFaces()){
                        if(face.getReferences().size()>3){
                            throw new Exception("Model at "+path+" not triangulated");
                        }
                        for(OBJDataReference reference : face.getReferences()) {
                            //System.out.println(reference.vertexIndex+" "+reference.texCoordIndex+" "+reference.normalIndex);
                            if (!usedRefs.containsKey(reference)) {
                                usedRefs.put(reference, indiceNo);
                                indices.add(indiceNo);
                                final OBJVertex vertex = model.getVertex(reference);
                                vertices.add(vertex.x);
                                vertices.add(vertex.y);
                                vertices.add(vertex.z);

                                final OBJNormal normal = model.getNormal(reference);
                                normals.add(normal.x);
                                normals.add(normal.y);
                                normals.add(normal.z);

                                final OBJTexCoord texCoord = model.getTexCoord(reference);
                                texCoords.add(texCoord.u);
                                texCoords.add(1 - texCoord.v);

                                indiceNo++;
                            } else {
                                indices.add(usedRefs.get(reference));
                            }
                        }
                    }
                    if(materials.containsKey(mesh.getMaterialName())){
                        MTLMaterial mat = materials.get(mesh.getMaterialName());
                        diffuseTexture = loadTexture(new FileHandle(getPath(mat.getDiffuseTexture())));
                    }else{
                        throw new Exception("Could not find mesh material: "+mesh.getMaterialName());
                    }

                    /*for(int i = 0; i<indices.size(); i++){
                        System.out.println(vertices.get(indices.get(i)*3)+" "+vertices.get(indices.get(i)*3+1)+" "+vertices.get(indices.get(i)*3+2));
                    }*/

                    meshes.add(new Mesh(convertIntegers(indices), convertFloats(vertices), convertFloats(texCoords),convertFloats(normals), diffuseTexture));
                }
            }

        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        } catch (Exception e) {
            e.printStackTrace();
        }


    }

    private static int[] convertIntegers(List<Integer> integers)
    {
        int[] ret = new int[integers.size()];
        Iterator<Integer> iterator = integers.iterator();
        for (int i = 0; i < ret.length; i++)
        {
            ret[i] = iterator.next().intValue();
        }
        return ret;
    }

    private static float[] convertFloats(List<Float> floats)
    {
        float[] ret = new float[floats.size()];
        Iterator<Float> iterator = floats.iterator();
        for (int i = 0; i < ret.length; i++)
        {
            ret[i] = iterator.next().floatValue();
        }
        return ret;
    }



    private String getPath(String name){
        return Gdx.files.local("/models/"+name).file().getAbsolutePath();
    }


    public void render(Matrix4f modelMatrix){
        shader.use();
        shader.setMat4("model", modelMatrix);
        for (Mesh mesh : this.meshes){
            mesh.render(shader);
        }
    }

    private int loadTexture(FileHandle handle){
        int texture = Gdx.gl.glGenTexture();
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_2D, texture);

        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_WRAP_S, Gdx.gl.GL_REPEAT);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_WRAP_T, Gdx.gl.GL_REPEAT);

        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_MIN_FILTER, Gdx.gl.GL_LINEAR);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_MAG_FILTER, Gdx.gl.GL_LINEAR);

        Texture tex = new Texture(handle);

        tex.getTextureData().prepare();


        Pixmap pixmap = tex.getTextureData().consumePixmap();

        Gdx.gl.glTexImage2D(Gdx.gl.GL_TEXTURE_2D, 0, pixmap.getGLInternalFormat(), pixmap.getWidth(),
                pixmap.getHeight(), 0, pixmap.getGLFormat(), pixmap.getGLType(),
                pixmap.getPixels());
        Gdx.gl.glGenerateMipmap(Gdx.gl.GL_TEXTURE_2D);
        pixmap.dispose();
        tex.dispose();
        return texture;
    }

}
