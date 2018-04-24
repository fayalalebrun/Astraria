package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g3d.Renderable;
import com.badlogic.gdx.graphics.profiling.GLProfiler;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;
import com.mygdx.game.simulation.Options;
import com.mygdx.game.simulation.PlacementManager;
import com.mygdx.game.simulation.SimulationObject;
import com.sun.media.jfxmediaimpl.MediaDisposer;
import org.joml.*;
import sun.security.provider.SHA;

import java.lang.Math;
import java.nio.Buffer;
import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.ArrayList;
import java.util.Queue;
import java.util.concurrent.LinkedBlockingQueue;

/**
 * Created by Fran on 3/14/2018.
 */
public class Renderer implements Disposable{

    private static final float MAXVIEWDISTANCE = 100000000000f;

    private static final float LOGDEPTHCONSTANT = 1f;

    private int screenWidth, screenHeight;

    private Camera camera;

    private Shader planetShader;

    private Shader starShader;

    private Shader billboardShader;

    private Shader lensGlowShader;

    private Shader pointShader;

    private Shader planetAtmoShader;

    private Shader skyboxShader;

    private Shader blackHoleShader;

    private Shader lineShader;

    private Transformation transformation;
    private static float FOV =(float)Math.toRadians(45f);


    private LightSourceManager lightSourceManager;

    private final Vector3f temp;

    private final Vector2f temp2f;

    private final Vector4f temp4f;

    private final Matrix4f combined;


    Queue<LensGlow> lensGlows;

    FloatBuffer outBuff;

    private Skybox skybox;

    private MousePicker picker;

    private PlacementManager placementManager;


    public Renderer(int screenWidth, int screenHeight) {
        this.screenWidth = screenWidth;
        this.screenHeight = screenHeight;
        this.combined = new Matrix4f();

        outBuff = BufferUtils.newFloatBuffer(1);


        this.temp = new Vector3f();

        new GLProfiler(Gdx.graphics).enable();
        transformation = new Transformation();

        camera = new Camera(0, 0, 10);


        planetShader = new Shader(Gdx.files.internal("shaders/default.vert"), Gdx.files.internal("shaders/default.frag"));

        starShader =  new Shader(Gdx.files.internal("shaders/sunShader.vert"), Gdx.files.internal("shaders/sunShader.frag"));

        billboardShader = new Shader(Gdx.files.internal("shaders/billboard.vert"), Gdx.files.internal("shaders/billboard.frag"));

        lensGlowShader = new Shader(Gdx.files.internal("shaders/lensGlow.vert"), Gdx.files.internal("shaders/lensGlow.frag"));

        pointShader = new Shader(Gdx.files.internal("shaders/point.vert"), Gdx.files.internal("shaders/point.frag"));

        planetAtmoShader = new Shader(Gdx.files.internal("shaders/planetAtmo.vert"), Gdx.files.internal("shaders/planetAtmo.frag"));

        skyboxShader = new Shader(Gdx.files.internal("shaders/skybox.vert"), Gdx.files.internal("shaders/skybox.frag"));

        blackHoleShader = new Shader(Gdx.files.internal("shaders/blackHole.vert"), Gdx.files.internal("shaders/blackHole.frag"));

        lineShader = new Shader(Gdx.files.internal("shaders/line.vert"), Gdx.files.internal("shaders/line.frag"));

        lensGlows = new LinkedBlockingQueue<LensGlow>();

        lightSourceManager = new LightSourceManager(camera, transformation);
        lightSourceManager.addShader(planetShader);
        lightSourceManager.addShader(planetAtmoShader);


        temp2f = new Vector2f();

        temp4f = new Vector4f();

        skybox = loadSkybox();

        picker = new MousePicker();

        placementManager = new PlacementManager(picker);
    }

    private Skybox loadSkybox(){
        FileHandle[] fileHandles = new FileHandle[]{
                Gdx.files.internal("skybox/MilkyWayXP.png"),
                Gdx.files.internal("skybox/MilkyWayXN.png"),
                Gdx.files.internal("skybox/MilkyWayYP.png"),
                Gdx.files.internal("skybox/MilkyWayYN.png"),
                Gdx.files.internal("skybox/MilkyWayZP.png"),
                Gdx.files.internal("skybox/MilkyWayZN.png")
        };

        int cubemap = Warehouse.getOpenGLTextureManager().loadCubeMap(fileHandles);

        return new Skybox(cubemap, skyboxShader);
    }

    public Camera getCamera() {
        return camera;
    }

    public void render(float delta, ArrayList<SimulationObject> toDraw){
        camera.update(delta);
        lightSourceManager.update();

        Gdx.gl.glEnable(Gdx.gl.GL_CULL_FACE);
        Gdx.gl.glEnable(Gdx.gl.GL_DEPTH_TEST);


        Gdx.gl.glClearColor(0.0f,0.0f,0.0f,1.0f);
        Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT | GL20.GL_DEPTH_BUFFER_BIT | (Gdx.graphics.getBufferFormat().coverageSampling?GL20.GL_COVERAGE_BUFFER_BIT_NV:0));



        Matrix4f projection = transformation.getProjectionMatrix(FOV, screenWidth,screenHeight,1f,MAXVIEWDISTANCE);
        combined.set(projection).mul(transformation.getViewMatrix(camera));
        Matrix4f view = transformation.getViewMatrix(camera);

        picker.update(view, projection, screenWidth, screenHeight);

        planetShader.use();
        planetShader.setFloat("og_farPlaneDistance", MAXVIEWDISTANCE);
        planetShader.setFloat("u_logarithmicDepthConstant", LOGDEPTHCONSTANT);
        planetShader.setMat4("projection", projection);

        starShader.use();
        starShader.setFloat("og_farPlaneDistance", MAXVIEWDISTANCE);
        starShader.setFloat("u_logarithmicDepthConstant", LOGDEPTHCONSTANT);
        starShader.setMat4("projection", projection);
        starShader.setMat4("view",view);

        billboardShader.use();
        billboardShader.setFloat("og_farPlaneDistance", MAXVIEWDISTANCE);
        billboardShader.setFloat("u_logarithmicDepthConstant", LOGDEPTHCONSTANT);
        billboardShader.setMat4("projection", projection);

        lensGlowShader.use();
        lensGlowShader.setMat4("projection", projection);
        lensGlowShader.setVec3f("camDir", camera.getFront());

        pointShader.use();
        pointShader.setFloat("og_farPlaneDistance", MAXVIEWDISTANCE);
        pointShader.setFloat("u_logarithmicDepthConstant", LOGDEPTHCONSTANT);
        pointShader.setMat4("projection", projection);



        planetAtmoShader.use();
        planetAtmoShader.setFloat("og_farPlaneDistance", MAXVIEWDISTANCE);
        planetAtmoShader.setFloat("u_logarithmicDepthConstant", LOGDEPTHCONSTANT);
        planetAtmoShader.setMat4("projection",projection);
        planetAtmoShader.setMat4("view",view);

        skyboxShader.use();
        skyboxShader.setMat4("projection",projection);
        skyboxShader.setMat4("view",view);

        blackHoleShader.use();
        blackHoleShader.setFloat("og_farPlaneDistance", MAXVIEWDISTANCE);
        blackHoleShader.setFloat("u_logarithmicDepthConstant", LOGDEPTHCONSTANT);
        blackHoleShader.setMat4("projection",projection);
        blackHoleShader.setMat4("view",view);

        lineShader.use();
        //lineShader.setFloat("og_farPlaneDistance", MAXVIEWDISTANCE);
        lineShader.setFloat("u_logarithmicDepthConstant", LOGDEPTHCONSTANT);
        lineShader.setMat4("projection",projection);
        lineShader.setMat4("view", view);
        lineShader.setFloat("FC",1.0f/(float)Math.log(MAXVIEWDISTANCE*LOGDEPTHCONSTANT + 1));

        placementManager.render(camera,delta);

        for(SimulationObject object : toDraw){
            object.render(camera,delta);
        }



        skybox.render();


        for(LensGlow lensGlow : lensGlows){
            lensGlow.checkVisibility();
        }


        Gdx.gl.glEnable(Gdx.gl.GL_BLEND);
        Gdx.gl.glBlendFunc(Gdx.gl.GL_SRC_ALPHA, Gdx.gl.GL_ONE);

        if(Options.drawOrbits) {
            for (SimulationObject object : toDraw) {
                object.getOrbit().render();
            }
        }



        while(!lensGlows.isEmpty()&&Options.drawLensGlow){
            try {
                lensGlows.poll().render();

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
        lensGlows.clear();

        Gdx.gl.glDisable(Gdx.gl.GL_BLEND);

        //simulationObject.render(camera);
        //simulationObject2.render(camera);
        Gdx.gl.glDisable(Gdx.gl.GL_CULL_FACE);
        Gdx.gl.glDisable(Gdx.gl.GL_DEPTH_TEST);

    }

    public Vector2f projectPoint(Vector3f position){

        return projectPoint(worldSpaceToDeviceCoords(temp4f.set(position.x,position.y,position.z, 1)));
    }

    public Vector2f projectPoint(Vector4f deviceCoords){
        temp4f.set(deviceCoords);
        temp4f.x*=screenWidth;
        temp4f.y*=screenHeight;

        //If the point is not visible, return null
        if (temp4f.w < 0){
            return temp2f.set(-1000,-1000);
        }
        return temp2f.set(temp4f.x,temp4f.y);
    }

    public float getFramebufferDepthComponent(int x, int y){
        Gdx.gl.glReadPixels(x,y,1,1,Gdx.gl.GL_DEPTH_COMPONENT,Gdx.gl.GL_FLOAT,outBuff);
        return outBuff.get(0);
    }


    public void updateScreenSize(int width, int height){
        screenWidth = width;
        screenHeight = height;
    }

    public Vector4f worldSpaceToDeviceCoords(Vector4f pos){
        temp4f.set(pos);
        Matrix4f projection = transformation.getProjectionMatrix(FOV, screenWidth,screenHeight,1f,MAXVIEWDISTANCE);
        Matrix4f view = transformation.getViewMatrix(camera);
        view.transform(temp4f); //Multiply the point vector by the view matrix
        projection.transform(temp4f); //Multiply the point vector by the projection matrix


        temp4f.x = ((temp4f.x / temp4f.w) + 1) / 2f; //Convert x coordinate to range between 0 to 1
        temp4f.y = ((temp4f.y / temp4f.w) + 1) / 2f; //Convert y coordinate to range between 0 to 1

        //Logarithmic depth buffer z-value calculation (Get rid of this if not using a logarithmic depth buffer)
        temp4f.z = ((2.0f * (float)Math.log(LOGDEPTHCONSTANT * temp4f.z + 1.0f) /
                (float)Math.log(LOGDEPTHCONSTANT * MAXVIEWDISTANCE + 1.0f)) - 1.0f) * temp4f.w;

        temp4f.z /= temp4f.w; //Perform perspective division on the z-value
        temp4f.z = (temp4f.z + 1)/2f; //Transform z coordinate into range 0 to 1

        return temp4f;
    }



    public Shader getPlanetShader() {
        return planetShader;
    }

    public Shader getStarShader() {
        return starShader;
    }

    public Shader getPointShader() {
        return pointShader;
    }

    public void addLight(LightEmitter light){
        lightSourceManager.addLight(light);
    }

    public LightSourceManager getLightSourceManager() {
        return lightSourceManager;
    }

    public Transformation getTransformation() {
        return transformation;
    }

    public Shader getLensGlowShader() {
        return lensGlowShader;
    }

    public int getScreenWidth() {
        return screenWidth;
    }

    public int getScreenHeight() {
        return screenHeight;
    }

    public void submitLensGlow(LensGlow lensGlow){
        lensGlows.add(lensGlow);
    }

    public Shader getPlanetAtmoShader() {
        return planetAtmoShader;
    }

    public Shader getBlackHoleShader() {
        return blackHoleShader;
    }

    public Skybox getSkybox() {
        return skybox;
    }

    public Shader getLineShader() {
        return lineShader;
    }

    public PlacementManager getPlacementManager() {
        return placementManager;
    }

    @Override
    public void dispose() {
        skybox.dispose();
    }
}
