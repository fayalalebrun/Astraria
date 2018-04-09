package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.BitmapFont;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.profiling.GLProfiler;
import com.badlogic.gdx.utils.BufferUtils;
import com.mygdx.game.BaseScreen;
import com.mygdx.game.Boot;
import com.mygdx.game.ViewportListener;
import com.mygdx.game.simulation.renderer.*;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.ArrayList;
import java.util.Queue;
import java.util.concurrent.LinkedBlockingQueue;

/**
 * Created by fraayala19 on 12/12/17.
 */
public class SimulationScreen extends BaseScreen {

    private final Renderer renderer;
    private final SimCamInputProcessor processor;
    private ArrayList<SimulationObject> simulationObjects;
    private final BitmapFont labelFont;
    private SpriteBatch fontBatch;
    private Vector2f temp;


    public SimulationScreen(Boot boot) {
        super(boot);


        renderer = new Renderer(Gdx.graphics.getWidth(), Gdx.graphics.getHeight());

        processor = new SimCamInputProcessor(renderer.getCamera());

        temp = new Vector2f();

        labelFont = Boot.manager.get("Euclid10.fnt", BitmapFont.class);
        fontBatch = new SpriteBatch();

        Gdx.input.setInputProcessor(processor);

        simulationObjects = new ArrayList<SimulationObject>();

        simulationObjects.add(new Star(149598000,0,0,renderer.getModelManager().loadModel("sphere3.obj", renderer.getTransformation()),
                renderer,695700,"Sun2",5500));

        /*simulationObjects.add(new SimulationObject(3,0,0,renderer.getModelManager().loadModel("sphere3.obj", renderer.getTransformation()),
                renderer.getPlanetShader(), 1,"earth"));
        simulationObjects.add(new SimulationObject(0,0,10000000000f,renderer.getModelManager().loadModel("sphere3.obj", renderer.getTransformation())
                ,renderer.getPlanetShader(), 1000000000f,"sun"));*/


    }



    @Override
    public void show() {


    }


    @Override
    public void render(float delta) {
        Gdx.gl.glClearColor(1f,1f,1f,1.0f);
        Gdx.gl.glClear(Gdx.gl.GL_COLOR_BUFFER_BIT|Gdx.gl.GL_DEPTH_BUFFER_BIT);

        renderer.render(delta, simulationObjects);


        fontBatch.begin();
        for(SimulationObject object : this.simulationObjects){
            temp = renderer.projectPoint(object.getPositionRelativeToCamera(renderer.getCamera()));
            if(temp!=null) {
                labelFont.draw(fontBatch, object.getName(), temp.x, temp.y);
            }
        }
        fontBatch.end();

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
        renderer.updateScreenSize(width, height);
        fontBatch = new SpriteBatch();
    }



    @Override
    public void dispose() {
    }

}
