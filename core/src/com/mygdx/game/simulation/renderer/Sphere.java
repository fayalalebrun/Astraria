package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.utils.Disposable;
import org.joml.Vector3f;

import java.util.ArrayList;

/**
 * Created by Fran on 4/10/2018.
 */
public class Sphere extends Model implements Disposable {
    int diffuseTexture;

    public Sphere(Transformation transformation, int diffuseTexture){
        super(new ArrayList<Mesh>(),transformation);
        this.diffuseTexture = diffuseTexture;
        Mesh mesh = Warehouse.getModelManager().loadModel("sphere.obj", transformation).getMeshes().get(0);
        mesh.setDiffuseTexture(diffuseTexture);

        meshes.add(mesh);

    }

    public Sphere(Transformation transformation, int diffuseTexture, boolean textured){
        this(transformation,diffuseTexture);
        meshes.get(0).setUseTexture(textured);

    }

    @Override
    public void render(Camera cam, Shader shader) {
        meshes.get(0).setDiffuseTexture(diffuseTexture);
        super.render(cam, shader);
    }
}
