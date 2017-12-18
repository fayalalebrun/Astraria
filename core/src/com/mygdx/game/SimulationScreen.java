package com.mygdx.game;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.assets.AssetManager;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.PerspectiveCamera;
import com.badlogic.gdx.graphics.VertexAttributes;
import com.badlogic.gdx.graphics.g2d.BitmapFont;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.g3d.*;
import com.badlogic.gdx.graphics.g3d.attributes.ColorAttribute;
import com.badlogic.gdx.graphics.g3d.attributes.FloatAttribute;
import com.badlogic.gdx.graphics.g3d.attributes.TextureAttribute;
import com.badlogic.gdx.graphics.g3d.environment.DirectionalLight;
import com.badlogic.gdx.graphics.g3d.utils.CameraInputController;
import com.badlogic.gdx.graphics.g3d.utils.ModelBuilder;
import com.badlogic.gdx.math.Quaternion;
import com.badlogic.gdx.math.Vector3;
import com.badlogic.gdx.math.collision.BoundingBox;
import com.badlogic.gdx.utils.Align;
import com.kotcrab.vis.ui.VisUI;
import com.kotcrab.vis.ui.widget.VisLabel;
import com.mygdx.game.logic.Body;
import com.mygdx.game.logic.DetailedBody;

import javax.xml.soap.Detail;
import java.util.ArrayList;
import java.util.Vector;

import static com.mygdx.game.Boot.manager;

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
    SpriteBatch bodyLabelBatch;

    DetailedBody earth;

    private CameraInputController camController;

    public SimulationScreen(Boot boot) {
        super(boot);
        bodies = new Vector<DetailedBody>();
        instances = new ArrayList<ModelInstance>();
        bodyLabelBatch = new SpriteBatch();
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
        cam.near = 0.00000000001f;
        cam.far = 1496000000000f;
        cam.update();

        camController = new CameraInputController(cam);
        Gdx.input.setInputProcessor(camController);

        ModelBuilder modelBuilder = new ModelBuilder();

        final Material material = new Material(ColorAttribute.createDiffuse(Color.GREEN));
        final long attributes = VertexAttributes.Usage.Position | VertexAttributes.Usage.Normal;
        model = modelBuilder.createSphere(1, 1, 1, 24, 24, material, attributes);



        earth = new DetailedBody(123,0,0,0,model, 60000000000f,"earth");

        addBody(earth);
        earth.centerCamera(cam);


    }


    @Override
    public void render(float delta) {


        for(DetailedBody body : this.bodies){
            body.render();
        }

        Gdx.gl.glViewport(0, 0, Gdx.graphics.getWidth(), Gdx.graphics.getHeight());
        Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT | GL20.GL_DEPTH_BUFFER_BIT);
        cam.update();
        modelBatch.begin(cam);
        modelBatch.render(instances, environment);
        modelBatch.end();

        drawLabels(bodyLabelBatch);
    }

    private void drawLabels(SpriteBatch batch){
        batch.begin();

        BitmapFont font = manager.get("comicsans.fnt", BitmapFont.class);

        Vector3 labelPos = earth.getPos().cpy();

        cam.project(labelPos);
        for(DetailedBody body : this.bodies){
            font.draw(bodyLabelBatch,body.getName(),labelPos.x,labelPos.y);
        }
        batch.end();
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
