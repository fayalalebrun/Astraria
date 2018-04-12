package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.BitmapFont;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.profiling.GLProfiler;
import com.badlogic.gdx.utils.BufferUtils;
import com.mygdx.game.BaseScreen;
import com.mygdx.game.Boot;
import com.mygdx.game.ViewportListener;
import com.mygdx.game.logic.Body;
import com.mygdx.game.logic.algorithms.NBodyAlgorithm;
import com.mygdx.game.logic.algorithms.VelocityVerlet;
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

    Thread algorithmThread;

    NBodyAlgorithm algorithm;

    private final Renderer renderer;
    private final SimCamInputProcessor processor;
    private ArrayList<SimulationObject> simulationObjects;
    private final BitmapFont labelFont;
    private SpriteBatch fontBatch;


    public SimulationScreen(Boot boot) {
        super(boot);
        Warehouse.init();

        algorithm = new VelocityVerlet();
        algorithmThread = new Thread(algorithm);
        algorithmThread.start();

        renderer = new Renderer(Gdx.graphics.getWidth(), Gdx.graphics.getHeight());

        processor = new SimCamInputProcessor(renderer.getCamera());

        labelFont = Boot.manager.get("Euclid10.fnt", BitmapFont.class);
        fontBatch = new SpriteBatch();

        Gdx.input.setInputProcessor(processor);

        simulationObjects = new ArrayList<SimulationObject>();

        addObject(new Star(new Sphere(renderer.getTransformation(),
                        Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("models/earth.jpg").path())),
                renderer,695700,"Sun2",renderer.getTransformation(),new Body(1.9891e30, 149598000000.0, 0, 0, 0, 0, 0),5500));



       addObject(new AtmospherePlanet(new Sphere(renderer.getTransformation(),
                Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("models/earth.jpg").path())),renderer.getPlanetAtmoShader(), 1,
                "atmo",renderer.getTransformation(), new Body(5.97219e24, 0, 0, 0, 0, 0,0),
                Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("atmoGradient.png").path()),
                renderer.getLightSourceManager(), Color.WHITE));

        addObject(new Star(new Sphere(renderer.getTransformation(),
                Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("models/earth.jpg").path())),
                renderer,695700,"Sun",renderer.getTransformation(),new Body(1.9891e30, -149598000000.0, 0, 0, 0, 0, 0),5500));

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
            Vector2f temp = renderer.projectPoint(object.getPositionRelativeToCamera(renderer.getCamera()));
            if(temp!=null) {
                labelFont.draw(fontBatch, object.getName(), temp.x, temp.y);
            }
        }
        fontBatch.end();

    }

    public void addObject(SimulationObject simulationObject){
        simulationObjects.add(simulationObject);
        algorithm.addBody(simulationObject.getBody());
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
        Warehouse.dispose();
        algorithmThread.interrupt();
    }

}
