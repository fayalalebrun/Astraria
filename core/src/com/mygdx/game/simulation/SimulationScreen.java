package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.g2d.BitmapFont;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.mygdx.game.BaseScreen;
import com.mygdx.game.Boot;
import com.mygdx.game.simulation.logic.Body;
import com.mygdx.game.simulation.logic.algorithms.NBodyAlgorithm;
import com.mygdx.game.simulation.logic.algorithms.VelocityVerlet;
import com.mygdx.game.simulation.renderer.*;
import org.joml.Vector2f;

import java.util.ArrayList;

/**
 * Created by fraayala19 on 12/12/17.
 */
public class SimulationScreen extends BaseScreen {

    public static float simSpeed = 1;

    Thread algorithmThread;

    NBodyAlgorithm algorithm;

    private final Renderer renderer;
    private final SimCamInputProcessor processor;
    private ArrayList<SimulationObject> simulationObjects;
    private final BitmapFont labelFont;
    private SpriteBatch fontBatch;


    public SimulationScreen(Boot boot, String loadPath) {
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

        /*addObject(new Star(new Sphere(renderer.getTransformation(),
                        Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("models/earth.jpg").path())),
                renderer,695700,"Sun2",renderer.getTransformation(),new Body(1.9891e30, 149598000000.0, 0, 0, 0, 0, 0),5500));



       addObject(new AtmospherePlanet(new Sphere(renderer.getTransformation(),
                Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("models/earth.jpg").path())),renderer.getPlanetAtmoShader(), 1,
                "atmo",renderer.getTransformation(), new Body(5.97219e24, 0, 0, 0, 0, 0,0),
                renderer.getLightSourceManager(), Color.WHITE));

        addObject(new Star(new Sphere(renderer.getTransformation(),
                Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("models/earth.jpg").path())),
                renderer,695700,"Sun",renderer.getTransformation(),new Body(1.9891e30, -149598000000.0, 0, 0, 0, 0, 0),5500));*/

        if(!loadPath.equals("")){
            SaveFileManager.loadGame(this,renderer,new FileHandle(loadPath));
        }

    }



    @Override
    public void show() {


    }


    @Override
    public void render(float delta) {
        for (SimulationObject simObj : simulationObjects){
            simObj.updatePosition();
        }


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

    public void clearObjects(){
        for(SimulationObject simulationObject : simulationObjects){
            algorithm.removeBody(simulationObject.getBody());
        }

        simulationObjects.clear();
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
