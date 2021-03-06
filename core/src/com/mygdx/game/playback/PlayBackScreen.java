package com.mygdx.game.playback;

import com.badlogic.gdx.*;
import com.badlogic.gdx.graphics.*;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.g3d.utils.FirstPersonCameraController;
import com.badlogic.gdx.math.Vector2;
import com.badlogic.gdx.math.Vector3;
import com.badlogic.gdx.scenes.scene2d.Group;
import com.badlogic.gdx.scenes.scene2d.InputListener;
import com.badlogic.gdx.scenes.scene2d.Stage;
import com.badlogic.gdx.utils.viewport.ScreenViewport;
import com.badlogic.gdx.utils.viewport.Viewport;
import com.kotcrab.vis.ui.widget.VisTable;
import com.kotcrab.vis.ui.widget.color.ColorPicker;
import com.kotcrab.vis.ui.widget.color.ColorPickerAdapter;
import com.mygdx.game.BaseScreen;
import com.mygdx.game.Boot;
import com.mygdx.game.simulation.logic.Body;
import com.mygdx.game.playback.ui.MenuWidget;
import com.mygdx.game.playback.ui.ProgressWindow;
import net.dermetfan.utils.Pair;

import java.security.Key;
import java.util.ArrayList;
import java.util.Vector;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * Created by fraayala19 on 1/11/18.
 */
public class PlayBackScreen extends BaseScreen{

    private PerspectiveCamera cam;



    private FirstPersonCameraController camControl;

    private SpriteBatch spriteBatch;

    private ArrayList<PlayBackBody> bodies;

    private int numberOfBodies;

    private float bodyScale, maxAccel, minAccel, bodyScaleMod=1;


    private float currTime = 0;

    private int currFrame, totalFrames;

    private Stage uiStage;

    private InputListener UIListener;

    private Group uiGroup;

    private ProgressWindow progressWindow;

    private boolean paused;

    private static Color upperColor = new Color(Color.ROYAL);
    private static Color lowerColor = new Color(Color.CLEAR);

    private ColorPicker upperColorPicker;
    private ColorPicker lowerColorPicker;

    private AtomicInteger timeMultiplier = new AtomicInteger(2);

    private float [] tempframe;

    private boolean halfSpeedCheck = false;

    Texture bodyTexture;

    OrthographicCamera testCamera;

    Viewport testViewport;

    PlayBackLoader2 playBackLoader;

    Thread loaderThread;

    float [] frame;

    private boolean initialized = false;

    private Vector3 frameVector;


    public PlayBackScreen(Boot boot, String arg) {
        super(boot);

        initialized=false;

        frameVector=new Vector3();

        frame = new float[4];

        bodyTexture = Boot.manager.get("particle.png");

        loadRecording(arg);

        UIListener = new InputListener();

        upperColorPicker = new ColorPicker("Color picker", new ColorPickerAdapter(){
            @Override
            public void finished (Color newColor) {
                setUpperColor(newColor);
            }
        });

        lowerColorPicker = new ColorPicker("Color picker", new ColorPickerAdapter(){
            @Override
            public void finished (Color newColor) {
                setLowerColor(newColor);
            }
        });

        upperColorPicker.fadeOut(0);
        lowerColorPicker.fadeOut(0);

        uiStage = new Stage(new ScreenViewport());
        uiStage.addListener(UIListener);

        uiGroup = new Group();

        final VisTable menuWidgteTable = new VisTable();

        menuWidgteTable.setFillParent(true);
        menuWidgteTable.left().top();
        uiStage.addActor(menuWidgteTable);

        menuWidgteTable.add(new MenuWidget(uiGroup, this ).getTable()).fillX().expandX().row();

        uiStage.addActor(uiGroup);





        bodies = new ArrayList<PlayBackBody>();

        cam = new PerspectiveCamera(67, Gdx.graphics.getWidth(), Gdx.graphics.getHeight());

        testCamera = new OrthographicCamera();
        testViewport = new ScreenViewport(testCamera);
        testViewport.apply();

        camControl = new FirstPersonCameraController(cam);
        camControl.setVelocity(1000f);

        InputMultiplexer multiplexer = new InputMultiplexer();
        multiplexer.addProcessor(new InputProcessor() {
            @Override
            public boolean keyDown(int keycode) {
                if (keycode == Input.Keys.H){
                    uiGroup.setVisible(!uiGroup.isVisible());
                    menuWidgteTable.setVisible(!menuWidgteTable.isVisible());
                    return true;
                }
                if (keycode == Input.Keys.F){
                    toggleFullscreen();
                    return true;
                }
                return false;
            }

            @Override
            public boolean keyUp(int keycode) {
                return false;
            }

            @Override
            public boolean keyTyped(char character) {
                return false;
            }

            @Override
            public boolean touchDown(int screenX, int screenY, int pointer, int button) {
                return false;
            }

            @Override
            public boolean touchUp(int screenX, int screenY, int pointer, int button) {
                return false;
            }

            @Override
            public boolean touchDragged(int screenX, int screenY, int pointer) {
                return false;
            }

            @Override
            public boolean mouseMoved(int screenX, int screenY) {
                return false;
            }

            @Override
            public boolean scrolled(int amount) {
                return false;
            }
        });
        multiplexer.addProcessor(uiStage);
        multiplexer.addProcessor(camControl);


        Gdx.input.setInputProcessor(multiplexer);

        progressWindow = new ProgressWindow(this);

        spriteBatch = new SpriteBatch();


        setWindowPosition();

        pause();
    }

    private void setWindowPosition(){
        progressWindow.setX((Gdx.graphics.getWidth()/2)-210);
        progressWindow.setY(30);
    }

    @Override
    public void show() {

        cam.near = 1f;
        cam.far = 500000f;
        cam.position.set(300,0,0);
        cam.lookAt(0,0,0);
        cam.update();


        uiGroup.addActor(progressWindow);
        uiGroup.addActor(upperColorPicker);
        uiGroup.addActor(lowerColorPicker);


    }

    public int getCurrFrame() {
        return currFrame;
    }

    public void toggleFullscreen(){
        if(!Gdx.graphics.isFullscreen()){
            Gdx.graphics.setFullscreenMode(Gdx.graphics.getDisplayMode());
        } else {
            Gdx.graphics.setWindowedMode(800, 600);
        }
    }

    @Override
    public void render(float delta) {

        if (!initialized){
            frame = playBackLoader.requestNextFrame();
            initialized = true;
        }

        uiStage.act(Math.min(Gdx.graphics.getDeltaTime(), 1 / 30f));
        camControl.update(delta);

        cam.update();
        testCamera.update();

        Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT);

        Gdx.gl.glEnable(GL20.GL_BLEND);
        Gdx.gl.glBlendFunc(GL20.GL_SRC_ALPHA, GL20.GL_ONE);

        if (!paused){
            currTime += delta;

            if (timeMultiplier.get()==2){
                tempframe = playBackLoader.requestNextFrame();
            }else if (timeMultiplier.get()==1){
                if (!halfSpeedCheck){
                    tempframe = playBackLoader.requestNextFrame();
                }
                halfSpeedCheck = !halfSpeedCheck;
            }else {
                playBackLoader.skipFrames(1);
                tempframe = playBackLoader.requestNextFrame();
            }


            if(tempframe == null){
                System.out.println(currFrame+" frame not available ");
                currTime=0;
            } else {
                frame = tempframe;
                currFrame++;
            }
        }
            spriteBatch.setBlendFunction(GL20.GL_SRC_ALPHA, GL20.GL_ONE);
            spriteBatch.begin();
            for(int i = 0; i < frame.length; i+=4){
                drawBody(frame[i]*100f, frame[i+1]*100f, frame[i+2]*100f, frame[i+3]);
            }
            spriteBatch.end();



        uiStage.draw();

        //if(currTime>1/60f){
           // if(playBackLoader.containsFrame (currFrame+(int)(currTime*60f))) {
             //   currFrame += (int) (currTime * 60f);
                //skip currTime*60f frames
             //   playBackLoader.skipFrames( (int) (currTime*60f));
                //currFrame++;
           // } else {
                //next frame
            //}
          //  currTime=0;
        //}

        if(currFrame>totalFrames-1){
            currFrame=totalFrames-1;
        }

    }

    private void drawBody(float x, float y, float z, float accel){
        frameVector.set(x,y,z);
        if(cam.frustum.pointInFrustum(frameVector)){
            spriteBatch.setColor(getGradientColor(getPercentage(accel)));
            Vector2 newPos = projectPos(frameVector);
            spriteBatch.draw(bodyTexture,newPos.x,newPos.y,bodyTexture.getWidth()*bodyScale*bodyScaleMod,bodyTexture.getHeight()*bodyScale*bodyScaleMod);
        }
    }

    private float getPercentage(float accel){
        float range = maxAccel-minAccel;
        return (accel-minAccel)/range;
    }

    private Vector2 projectPos(Vector3 oldPos){
        Vector3 pos = oldPos.cpy();
        cam.project(pos);
        pos.x = pos.x - bodyTexture.getWidth()*bodyScale*bodyScaleMod*0.5f;
        pos.y = pos.y - bodyTexture.getHeight()*bodyScale*bodyScaleMod*0.5f;
        return new Vector2(pos.x,pos.y);
    }

    @Override
    public void resize(int width, int height) {
        cam.viewportHeight = height;
        cam.viewportWidth = width;
        spriteBatch = new SpriteBatch();
        testViewport.update(width,height,true);
        uiStage.getViewport().update(width,height, true);
        setWindowPosition();
    }


    public void pause() {
        setPaused(true);
    }

    @Override
    public void resume() {

    }

    @Override
    public void hide() {

    }

    @Override
    public void dispose() {
        spriteBatch.dispose();
        uiStage.dispose();
        upperColorPicker.dispose();
        lowerColorPicker.dispose();
        playBackLoader.terminate();
    }


    public int getTotalFrames() {
        return totalFrames;
    }

    private void loadRecording(String path){

        playBackLoader = new PlayBackLoader2(path);
        loaderThread = new Thread(playBackLoader);
        loaderThread.start();

        totalFrames = playBackLoader.getCycles();
        numberOfBodies = playBackLoader.getNumberOfBodies();
        maxAccel = playBackLoader.getMaxAccel();
        minAccel = playBackLoader.getMinAccel();
        bodyScale = playBackLoader.getBodyScale()*0.01f;
    }


    public void setCurrFrame(int currFrame) {
        this.currFrame = currFrame;
        playBackLoader.changeFramePosition(currFrame);
    }

    public void setPaused(boolean paused) {
        this.paused = paused;
    }

    public float getMaxAccel() {
        return maxAccel;
    }

    public float getMinAccel() {
        return minAccel;
    }

    public static Color getGradientColor( float percent){
        return new Color(upperColor.r*percent+lowerColor.r*(1-percent),
                upperColor.g*percent+lowerColor.g*(1-percent),
                upperColor.b*percent+lowerColor.b*(1-percent),
                1);

    }

    public void setUpperColor(Color color){
        upperColor.set(color);
    }

    public void setLowerColor(Color color) {
        lowerColor.set(color);
    }

    public ColorPicker getUpperColorPicker(){
        return upperColorPicker;
    }

    public ColorPicker getLowerColorPicker(){
        return lowerColorPicker;
    }

    public Group getUiGroup(){
        return uiGroup;
    }

    public ArrayList<PlayBackBody> getBodies(){
        return bodies;
    }

    public void setTimeMultiplier (int value){
        timeMultiplier.set(value);
    }

    public float getBodyScaleMod() {
        return bodyScaleMod;
    }

    public void setBodyScaleMod(float bodyScaleMod) {
        this.bodyScaleMod = bodyScaleMod;
    }
    public void setCamVelocity(float value){
        camControl.setVelocity(value);
    }

    public PlayBackLoader2 getPlayBackLoader() {
        return playBackLoader;
    }
}
