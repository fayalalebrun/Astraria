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

    private OpenGLTextureManager textures;
    private ArrayList<Mesh> meshes;
    private Shader shader;
    private final Vector3f position, rotation;
    private final Transformation transformation;
    private float scale;


    public Model(OpenGLTextureManager textureManager, String name, Shader shader, Transformation transformation, Vector3f position, Vector3f rotation, float scale) {
        this.position = position;
        this.rotation = rotation;
        this.textures = textureManager;
        this.scale = scale;
        meshes = new ArrayList<Mesh>();
        this.shader = shader;
        this.transformation = transformation;
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
                        if(face.getReferences().size()!=3){
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
                        diffuseTexture = textures.addTexture(getPath(mat.getDiffuseTexture()));

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
