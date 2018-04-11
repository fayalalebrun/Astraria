package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.utils.Disposable;
import org.joml.Vector3f;

import java.util.ArrayList;

/**
 * Created by Fran on 4/10/2018.
 */
public class Sphere extends Model implements Disposable {


    public Sphere(ModelManager modelManager, Transformation transformation, int diffuseTexture){
        super(new ArrayList<Mesh>(),transformation);

        Mesh mesh = modelManager.loadModel("sphere.obj", transformation).getMeshes().get(0).copy();
        mesh.setDiffuseTexture(diffuseTexture);

        meshes.add(mesh);
    }

    public Sphere(ModelManager modelManager, Transformation transformation, int diffuseTexture, boolean textured){
        this(modelManager,transformation,diffuseTexture);
        meshes.get(0).setUseTexture(textured);

    }

}
