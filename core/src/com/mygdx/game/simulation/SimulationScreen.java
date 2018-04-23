package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.InputMultiplexer;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.g2d.BitmapFont;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.scenes.scene2d.Stage;
import com.badlogic.gdx.utils.viewport.ScreenViewport;
import com.kotcrab.vis.ui.widget.Menu;
import com.kotcrab.vis.ui.widget.MenuBar;
import com.kotcrab.vis.ui.widget.VisTable;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.BaseScreen;
import com.mygdx.game.Boot;
import com.mygdx.game.simulation.logic.Body;
import com.mygdx.game.simulation.logic.algorithms.NBodyAlgorithm;
import com.mygdx.game.simulation.logic.algorithms.VelocityVerlet;
import com.mygdx.game.simulation.renderer.*;
import com.mygdx.game.simulation.ui.SimulationScreenInterface;
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
    private final PlacementInputProcessor placeProcessor;
    private ArrayList<SimulationObject> simulationObjects;
    private final BitmapFont labelFont;
    private SpriteBatch fontBatch;
    private final ArrayList<Clickable3DObject> clickable3DObjects;

    private InputMultiplexer multiplexer;

    private final SimulationScreenInterface simulationScreenInterface;



    public SimulationScreen(Boot boot, String loadPath) {
        super(boot);
        Warehouse.init();

        FileHandle dirHandle = Gdx.files.internal("./Planet Textures/");
        for(FileHandle f : dirHandle.list()){
            if(f.extension().equals("jpg")||f.extension().equals("png")){
                Warehouse.getOpenGLTextureManager().addTexture(f);
            }
        }

        multiplexer = new InputMultiplexer();

        algorithm = new VelocityVerlet();
        algorithmThread = new Thread(algorithm);
        algorithmThread.start();

        renderer = new Renderer(Gdx.graphics.getWidth(), Gdx.graphics.getHeight());

        processor = new SimCamInputProcessor(renderer.getCamera());
        placeProcessor = new PlacementInputProcessor(renderer.getCamera(), renderer.getPlacementManager(), this);


        labelFont = Boot.manager.get("Euclid10.fnt", BitmapFont.class);
        fontBatch = new SpriteBatch();

        Gdx.input.setInputProcessor(multiplexer);

        simulationObjects = new ArrayList<SimulationObject>();

        this.simulationScreenInterface = new SimulationScreenInterface(this,multiplexer,algorithm);
        multiplexer.addProcessor(placeProcessor);
        multiplexer.addProcessor(processor);


        clickable3DObjects = new ArrayList<Clickable3DObject>();

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
        simulationScreenInterface.update();

        Gdx.gl.glClearColor(1f,1f,1f,1.0f);
        Gdx.gl.glClear(Gdx.gl.GL_COLOR_BUFFER_BIT|Gdx.gl.GL_DEPTH_BUFFER_BIT);

        renderer.render(delta, simulationObjects);

        clickable3DObjects.clear();
        for (SimulationObject simObj : simulationObjects){
            simObj.getClickable3DObject().updateCoords(renderer);
            if (simObj.getClickable3DObject().getDeviceCoords().w>0) {
                clickable3DObjects.add(simObj.getClickable3DObject());
            }
        }

        processor.updateClickableObjects(clickable3DObjects);

        fontBatch.begin();
        for(SimulationObject object : this.simulationObjects){
            Vector2f temp = renderer.projectPoint(object.getPositionRelativeToCamera(renderer.getCamera()));
            if(temp!=null) {
                //labelFont.setColor(object.getOrbitColor());
                labelFont.draw(fontBatch, object.getName(), temp.x, temp.y);
            }
        }
        fontBatch.end();


        simulationScreenInterface.render();
    }

    public void addObject(SimulationObject simulationObject){
        simulationObjects.add(simulationObject);
        algorithm.addBody(simulationObject.getBody());
        simulationScreenInterface.addObject(simulationObject);
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
        simulationScreenInterface.resize(width,height);
    }

    public Renderer getRenderer() {
        return renderer;
    }

    @Override
    public void dispose() {
        Warehouse.dispose();
        algorithmThread.interrupt();
    }

    public ArrayList<SimulationObject> getSimulationObjects() {
        return simulationObjects;
    }
}
