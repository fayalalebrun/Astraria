package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.utils.Disposable;
import com.mokiat.data.front.parser.*;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.util.*;

/**
 * Created by Fran on 3/19/2018.
 */
public class ModelManager implements Disposable{

    final OpenGLTextureManager textures;
    final Map<String, Model> models;
    final ArrayList<Model> extraModels;

    public ModelManager(OpenGLTextureManager openGLTextureManager) {
        this.textures = openGLTextureManager;
        models = new HashMap<String, Model>();
        extraModels = new ArrayList<Model>();
    }

    public Model loadModel(String name, Transformation transformation){
        if(models.containsKey(name)){
            return models.get(name);
        }
        String path = getPath(name);
        ArrayList<Mesh> meshes = new ArrayList<Mesh>();
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
                    GLTexture diffuseTexture = null;

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

                    diffuseTexture = null;

                    if (materials.containsKey(mesh.getMaterialName())) {
                        MTLMaterial mat = materials.get(mesh.getMaterialName());
                        diffuseTexture = textures.addTexture(Gdx.files.absolute(getPath(mat.getDiffuseTexture())));
                    }

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
        Model m = new Model(meshes, transformation);
        models.put(name, m);
        return m;
    }

    private String getPath(String name){
        return Gdx.files.local("/models/"+name).file().getAbsolutePath();
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

    public void addModel(Model model){
        extraModels.add(model);
    }


    @Override
    public void dispose() {
        for(Model model : this.models.values()){
            model.dispose();
        }

        for (Model model : extraModels){
            model.dispose();
        }
    }
}
