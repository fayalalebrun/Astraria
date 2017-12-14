package com.mygdx.game;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.PerspectiveCamera;
import com.badlogic.gdx.graphics.VertexAttributes;
import com.badlogic.gdx.graphics.g3d.*;
import com.badlogic.gdx.graphics.g3d.attributes.ColorAttribute;
import com.badlogic.gdx.graphics.g3d.attributes.FloatAttribute;
import com.badlogic.gdx.graphics.g3d.attributes.TextureAttribute;
import com.badlogic.gdx.graphics.g3d.environment.DirectionalLight;
import com.badlogic.gdx.graphics.g3d.utils.CameraInputController;
import com.badlogic.gdx.graphics.g3d.utils.ModelBuilder;
import com.badlogic.gdx.math.Quaternion;
import com.badlogic.gdx.math.Vector3;
import com.mygdx.game.logic.Body;
import com.mygdx.game.logic.DetailedBody;

import java.util.ArrayList;
import java.util.Vector;

/**
 * Created by fraayala19 on 12/12/17.
 */
public class SimulationScreen extends BaseScreen {

    private PerspectiveCamera cam;

    private ModelBatch modelBatch;
    private ModelInstance instance;
    private Model model;
    private Environment environment;
    ArrayList<ModelInstance> instances;
    Vector<DetailedBody> bodies;

    private CameraInputController camController;

    public SimulationScreen(Boot boot) {
        super(boot);
        bodies = new Vector<DetailedBody>();
        instances = new ArrayList<ModelInstance>();
    }

    @Override
    public void show() {

        environment = new Environment();
        environment.set(new ColorAttribute(ColorAttribute.AmbientLight, 0.4f, 0.4f, 0.4f, 1f));
        environment.add(new DirectionalLight().set(0.8f, 0.8f, 0.8f, -1f, -0.8f, -0.2f));

        modelBatch = new ModelBatch();

        cam = new PerspectiveCamera(67, Gdx.graphics.getWidth(), Gdx.graphics.getHeight());
        cam.position.set(0.0000000668449198f, 0.0000000668449198f, 0.0000000668449198f);
        cam.lookAt(0,0,0);
        cam.near = 0f;
        cam.far = 14960000000000000f;
        cam.update();

        camController = new CameraInputController(cam);
        Gdx.input.setInputProcessor(camController);

        ModelBuilder modelBuilder = new ModelBuilder();

        final Material material = new Material(ColorAttribute.createDiffuse(Color.GREEN));
        final long attributes = VertexAttributes.Usage.Position | VertexAttributes.Usage.Normal;
        model = modelBuilder.createSphere(0.00000000668449198f, 0.00000000668449198f, 0.00000000668449198f, 24, 24, material, attributes);
        instance = new ModelInstance(model);
        instances.add(instance);


    }


    @Override
    public void render(float delta) {
        Gdx.gl.glViewport(0, 0, Gdx.graphics.getWidth(), Gdx.graphics.getHeight());
        Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT | GL20.GL_DEPTH_BUFFER_BIT);
        cam.update();
        modelBatch.begin(cam);
        modelBatch.render(instances, environment);
        modelBatch.end();
    }


    @Override
    public void resume() {

    }

    @Override
    public void hide() {

    }

    @Override
    public void pause() {

    }

    @Override
    public void resize(int width, int height) {
        cam.viewportHeight = height;
        cam.viewportWidth = width;
    }


    @Override
    public void dispose() {
        modelBatch.dispose();
        model.dispose();
    }

    public void addBody(DetailedBody body){
        instances.add(body.getInstance());
        bodies.add(body);
    }

    public void removeBody(DetailedBody body){
        instances.remove(body);
        bodies.remove(body);
    }


}
