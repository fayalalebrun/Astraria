package com.mygdx.game.playback;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.InputMultiplexer;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.*;
import com.badlogic.gdx.graphics.g2d.Sprite;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.graphics.g3d.Material;
import com.badlogic.gdx.graphics.g3d.Model;
import com.badlogic.gdx.graphics.g3d.ModelBatch;
import com.badlogic.gdx.graphics.g3d.ModelInstance;
import com.badlogic.gdx.graphics.g3d.attributes.ColorAttribute;
import com.badlogic.gdx.graphics.g3d.decals.CameraGroupStrategy;
import com.badlogic.gdx.graphics.g3d.decals.Decal;
import com.badlogic.gdx.graphics.g3d.decals.DecalBatch;
import com.badlogic.gdx.graphics.g3d.decals.DecalMaterial;
import com.badlogic.gdx.graphics.g3d.utils.FirstPersonCameraController;
import com.badlogic.gdx.graphics.g3d.utils.ModelBuilder;
import com.badlogic.gdx.math.Matrix4;
import com.badlogic.gdx.math.Vector2;
import com.badlogic.gdx.math.Vector3;
import com.badlogic.gdx.scenes.scene2d.Group;
import com.badlogic.gdx.scenes.scene2d.InputListener;
import com.badlogic.gdx.scenes.scene2d.Stage;
import com.badlogic.gdx.scenes.scene2d.ui.Image;
import com.badlogic.gdx.utils.viewport.ExtendViewport;
import com.badlogic.gdx.utils.viewport.FitViewport;
import com.badlogic.gdx.utils.viewport.ScreenViewport;
import com.badlogic.gdx.utils.viewport.Viewport;
import com.kotcrab.vis.ui.widget.Menu;
import com.kotcrab.vis.ui.widget.MenuBar;
import com.kotcrab.vis.ui.widget.VisTable;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.kotcrab.vis.ui.widget.color.ColorPicker;
import com.kotcrab.vis.ui.widget.color.ColorPickerAdapter;
import com.mygdx.game.BaseScreen;
import com.mygdx.game.Boot;
import com.mygdx.game.logic.Body;
import com.mygdx.game.playback.ui.MenuWidget;
import com.mygdx.game.playback.ui.ProgressWindow;
import net.dermetfan.utils.Pair;

import java.io.FileInputStream;
import java.io.ObjectInputStream;
import java.lang.reflect.Array;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.Vector;

import static com.badlogic.gdx.graphics.GL20.GL_BLEND;
import static com.badlogic.gdx.graphics.GL20.GL_ONE_MINUS_SRC_ALPHA;
import static com.badlogic.gdx.graphics.GL20.GL_SRC_ALPHA;

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

    private float timeMultiplier =1 ;

    Texture bodyTexture;

    OrthographicCamera testCamera;

    Viewport testViewport;

    PlayBackLoader playBackLoader;

    Thread loaderThread;

    Vector<Pair<Vector3, Float>> frame;


    public PlayBackScreen(Boot boot, String arg) {
        super(boot);

        frame = new Vector<Pair<Vector3, Float>>();

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

        VisTable table = new VisTable();

        table.setFillParent(true);
        table.left().top();
        uiStage.addActor(table);

        table.add(new MenuWidget(uiGroup, this ).getTable()).fillX().expandX().row();

        uiStage.addActor(uiGroup);






        bodies = new ArrayList<PlayBackBody>();

        cam = new PerspectiveCamera(67, Gdx.graphics.getWidth(), Gdx.graphics.getHeight());

        testCamera = new OrthographicCamera();
        testViewport = new ScreenViewport(testCamera);
        testViewport.apply();

        camControl = new FirstPersonCameraController(cam);
        camControl.setVelocity(1000f);

        InputMultiplexer multiplexer = new InputMultiplexer();
        multiplexer.addProcessor(uiStage);
        multiplexer.addProcessor(camControl);

        Gdx.input.setInputProcessor(multiplexer);

        progressWindow = new ProgressWindow(this);

        spriteBatch = new SpriteBatch();


        setWindowPosition();


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

    @Override
    public void render(float delta) {
        if(!paused) {
            currTime += delta*timeMultiplier;
        }

        uiStage.act(Math.min(Gdx.graphics.getDeltaTime(), 1 / 30f));
        camControl.update(delta);

        cam.update();
        testCamera.update();

        Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT);

        Gdx.gl.glEnable(GL20.GL_BLEND);
        Gdx.gl.glBlendFunc(GL20.GL_SRC_ALPHA, GL20.GL_ONE);
        Vector<Pair<Vector3, Float>> tempframe = playBackLoader.requestFrame(currFrame);

        if(tempframe == null){
            System.out.println(currFrame+" frame not available ");
            currTime=0;
        } else {
            frame = tempframe;
        }
            spriteBatch.setBlendFunction(GL20.GL_SRC_ALPHA, GL20.GL_ONE);
            spriteBatch.begin();
            for(Pair<Vector3,Float> p : frame){
                drawBody(p.getKey(),p.getValue());
            }
            spriteBatch.end();



        uiStage.draw();


        if(currTime>1/60){
            currFrame+=(int)(currTime*60);
            currTime=0;
        }

        if(currFrame>totalFrames-1){
            currFrame=totalFrames-1;
        }

    }

    private void drawBody(Vector3 pos, float accel){
        if(cam.frustum.pointInFrustum(pos)){
            spriteBatch.setColor(getGradientColor(getPercentage(accel)));
            Vector2 newPos = projectPos(pos);
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

    @Override
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
        playBackLoader = new PlayBackLoader(path);
        loaderThread = new Thread(playBackLoader);
        loaderThread.start();

        totalFrames = playBackLoader.getCycles();
        numberOfBodies = playBackLoader.getNumberOfBodies();
        maxAccel = playBackLoader.getMaxAccel();
        minAccel = playBackLoader.getMinAccel();
        bodyScale = playBackLoader.getBodyScale()*0.01f;
    }

    public void setCurrTime(float currTime) {
        this.currTime = currTime;
    }

    public void setCurrFrame(int currFrame) {
        this.currFrame = currFrame;
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

    public void setTimeMultiplier (float value){
        timeMultiplier = value;
    }

    public float getBodyScaleMod() {
        return bodyScaleMod;
    }

    public void setBodyScaleMod(float bodyScaleMod) {
        this.bodyScaleMod = bodyScaleMod;
    }
}
