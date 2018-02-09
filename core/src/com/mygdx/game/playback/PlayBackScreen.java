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

import java.io.FileInputStream;
import java.io.ObjectInputStream;
import java.lang.reflect.Array;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.ArrayList;
import java.util.Comparator;

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

    private float bodyScale, maxAccel, minAccel;


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

    private int timeMultiplier;



    OrthographicCamera testCamera;

    Viewport testViewport;


    public PlayBackScreen(Boot boot, String arg) {
        super(boot);


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

        timeMultiplier = 60;

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

        loadRecording(arg);
    }

    private void setWindowPosition(){
        progressWindow.setX((Gdx.graphics.getWidth()/2)-210);
        progressWindow.setY(30);
    }

    @Override
    public void show() {

        cam.near = 1f;
        cam.far = 100000f;
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
            currFrame = (int) (currTime * timeMultiplier);

            currTime += delta;
        }


        uiStage.act(Math.min(Gdx.graphics.getDeltaTime(), 1 / 30f));
        camControl.update(delta);

        cam.update();
        testCamera.update();

        Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT);

        Gdx.gl.glEnable(GL20.GL_BLEND);
        Gdx.gl.glBlendFunc(GL20.GL_SRC_ALPHA, GL20.GL_ONE);

        spriteBatch.setBlendFunction(GL20.GL_SRC_ALPHA, GL20.GL_ONE);
        spriteBatch.begin();
        for(PlayBackBody body : this.bodies) {
            body.setFrame(currFrame, cam, spriteBatch,minAccel,maxAccel);
        }
        spriteBatch.end();

        uiStage.draw();

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
        setPaused(false);
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
    }


    public int getTotalFrames() {
        return totalFrames;
    }

    private void loadRecording(String path){
        try {
            FileInputStream ifStream = new FileInputStream(path);
            ObjectInputStream stream = new ObjectInputStream(ifStream);

            short version = stream.readShort();
            if(version==1){
                numberOfBodies = stream.readInt();
                bodyScale = stream.readFloat();
                for(int i = 0; i < numberOfBodies; i++){
                    bodies.add(new PlayBackBody(new Sprite(Boot.manager.get("particle.png",
                            Texture.class)),bodyScale));
                }

                while (ifStream.available()>15){
                    //System.out.println(ifStream.available());
                    for(int i = 0; i < numberOfBodies; i++){
                        float x = stream.readFloat()*100;
                        float y = stream.readFloat()*100;
                        float z = stream.readFloat()*100;
                        bodies.get(i).addPosition(new Vector3(x,y,z));
                        bodies.get(i).addAcceleration(stream.readFloat());
                    }
                }
                if (stream.available()>=8){
                    maxAccel = stream.readFloat();
                    minAccel = 0;
                }else {
                    maxAccel=1;
                    minAccel=0;
                }


            }

            stream.close();

        }catch (Exception e){
            e.printStackTrace();
        }

        if(bodies.size()>0){
            totalFrames = bodies.get(0).getPositions().size();
        }
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

    public void setTimeMultiplier (int value){
        timeMultiplier = value;
    }
}
