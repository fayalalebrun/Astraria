package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g3d.Renderable;
import com.badlogic.gdx.graphics.profiling.GLProfiler;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;
import com.mygdx.game.simulation.SimulationObject;
import com.sun.media.jfxmediaimpl.MediaDisposer;
import org.joml.*;

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

    private Shader skyFromSpace;

    private Shader groundFromSpace;

    private Shader planetAtmoShader;

    private Transformation transformation;
    private static float FOV =(float)Math.toRadians(45f);


    private LightSourceManager lightSourceManager;

    private final Vector3f temp;

    private final Vector2f temp2f;

    private final Vector4f temp4f;

    private final Matrix4f combined;



    Queue<LensGlow> lensGlows;

    FloatBuffer outBuff;



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

        skyFromSpace = new Shader(Gdx.files.internal("shaders/SkyFromSpace.vert"), Gdx.files.internal("shaders/SkyFromSpace.frag"));

        groundFromSpace = new Shader(Gdx.files.internal("shaders/GroundFromSpace.vert"), Gdx.files.internal("shaders/GroundFromSpace.frag"));

        planetAtmoShader = new Shader(Gdx.files.internal("shaders/planetAtmo.vert"), Gdx.files.internal("shaders/planetAtmo.frag"));

        lensGlows = new LinkedBlockingQueue<LensGlow>();

        lightSourceManager = new LightSourceManager(camera, transformation);
        lightSourceManager.addShader(planetShader);
        lightSourceManager.addShader(planetAtmoShader);

        planetShader.use();

        try {
            planetShader.createUniform("diffuseTex");
            planetShader.createUniform("projection");
            planetShader.createUniform("modelView");
            planetShader.createUniform("og_farPlaneDistance");
            planetShader.createUniform("u_logarithmicDepthConstant");

        } catch (Exception e) {
            e.printStackTrace();
        }

        starShader.use();

        try {
            starShader.createUniform("diffuseTex");
            starShader.createUniform("projection");
            starShader.createUniform("modelView");
            starShader.createUniform("og_farPlaneDistance");
            starShader.createUniform("u_logarithmicDepthConstant");
            //starShader.createUniform("unDT");

        } catch (Exception e) {
            e.printStackTrace();
        }

        billboardShader.use();

        try {
            billboardShader.createUniform("billboardWidth");
            billboardShader.createUniform("billboardHeight");
            billboardShader.createUniform("screenWidth");
            billboardShader.createUniform("screenHeight");
            billboardShader.createUniform("billboardOrigin");

            billboardShader.createUniform("modelView");
            billboardShader.createUniform("projection");
            billboardShader.createUniform("og_farPlaneDistance");
            billboardShader.createUniform("u_logarithmicDepthConstant");

            billboardShader.createUniform("tex");


        } catch (Exception e) {
            e.printStackTrace();
        }

        lensGlowShader.use();

        try {
            lensGlowShader.createUniform("width");
            lensGlowShader.createUniform("height");
            lensGlowShader.createUniform("screenWidth");
            lensGlowShader.createUniform("screenHeight");
            lensGlowShader.createUniform("uPos");
            lensGlowShader.createUniform("temperature");

            lensGlowShader.createUniform("modelView");
            lensGlowShader.createUniform("projection");

            lensGlowShader.createUniform("tex");
            lensGlowShader.createUniform("spectrumTex");

            lensGlowShader.createUniform("camDir");


        } catch (Exception e) {
            e.printStackTrace();
        }

        pointShader.use();

        try {
            pointShader.createUniform("projection");
            pointShader.createUniform("modelView");
            pointShader.createUniform("og_farPlaneDistance");
            pointShader.createUniform("u_logarithmicDepthConstant");
            pointShader.createUniform("color");
        } catch (Exception e) {
            e.printStackTrace();
        }

        skyFromSpace.use();

        try{
            skyFromSpace.createUniform("modelView");
            skyFromSpace.createUniform("projection");

            skyFromSpace.createUniform("v3LightPos");
            skyFromSpace.createUniform("v3InvWavelength");
            //skyFromSpace.createUniform("fCameraHeight");
            skyFromSpace.createUniform("fCameraHeight2");
            skyFromSpace.createUniform("fOuterRadius");
            skyFromSpace.createUniform("fOuterRadius2");
            skyFromSpace.createUniform("fInnerRadius");
            //skyFromSpace.createUniform("fInnerRadius2");
            skyFromSpace.createUniform("fKrESun");
            skyFromSpace.createUniform("fKmESun");
            skyFromSpace.createUniform("fKr4PI");
            skyFromSpace.createUniform("fKm4PI");
            skyFromSpace.createUniform("fScale");
            skyFromSpace.createUniform("fScaleDepth");
            skyFromSpace.createUniform("fScaleOverScaleDepth");
            skyFromSpace.createUniform("og_farPlaneDistance");
            skyFromSpace.createUniform("u_logarithmicDepthConstant");
            skyFromSpace.createUniform("g");
            skyFromSpace.createUniform("g2");

        } catch (Exception e){
            e.printStackTrace();
        }


        groundFromSpace.use();

        try {
            groundFromSpace.createUniform("modelView");
            groundFromSpace.createUniform("projection");

            groundFromSpace.createUniform("v3LightPos");
            groundFromSpace.createUniform("v3InvWavelength");
            //groundFromSpace.createUniform("fCameraHeight");
            groundFromSpace.createUniform("fCameraHeight2");
            groundFromSpace.createUniform("fOuterRadius");
            groundFromSpace.createUniform("fOuterRadius2");
            groundFromSpace.createUniform("fInnerRadius");
            //groundFromSpace.createUniform("fInnerRadius2");
            groundFromSpace.createUniform("fKrESun");
            groundFromSpace.createUniform("fKmESun");
            groundFromSpace.createUniform("fKr4PI");
            groundFromSpace.createUniform("fKm4PI");
            groundFromSpace.createUniform("fScale");
            groundFromSpace.createUniform("fScaleDepth");
            groundFromSpace.createUniform("fScaleOverScaleDepth");
            groundFromSpace.createUniform("og_farPlaneDistance");
            groundFromSpace.createUniform("u_logarithmicDepthConstant");

            groundFromSpace.createUniform("diffuseTex");
        } catch (Exception e){
            e.printStackTrace();
        }

        temp2f = new Vector2f();

        temp4f = new Vector4f();
    }

    public Camera getCamera() {
        return camera;
    }

    public void render(float delta, ArrayList<SimulationObject> toDraw){
        camera.update(delta);
        lightSourceManager.update();

        Gdx.gl.glEnable(Gdx.gl.GL_CULL_FACE);
        Gdx.gl.glEnable(Gdx.gl.GL_DEPTH_TEST);
        Gdx.gl.glEnable(Gdx.gl.GL_BLEND);
        Gdx.gl.glBlendFunc(Gdx.gl.GL_SRC_ALPHA, Gdx.gl.GL_ONE);

        Gdx.gl.glClearColor(0.0f,0.0f,0.0f,1.0f);
        Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT | GL20.GL_DEPTH_BUFFER_BIT | (Gdx.graphics.getBufferFormat().coverageSampling?GL20.GL_COVERAGE_BUFFER_BIT_NV:0));



        Matrix4f projection = transformation.getProjectionMatrix(FOV, screenWidth,screenHeight,1f,MAXVIEWDISTANCE);
        combined.set(projection).mul(transformation.getViewMatrix(camera));

        planetShader.use();
        planetShader.setFloat("og_farPlaneDistance", MAXVIEWDISTANCE);
        planetShader.setFloat("u_logarithmicDepthConstant", LOGDEPTHCONSTANT);
        planetShader.setMat4("projection", projection);

        starShader.use();
        starShader.setFloat("og_farPlaneDistance", MAXVIEWDISTANCE);
        starShader.setFloat("u_logarithmicDepthConstant", LOGDEPTHCONSTANT);
        starShader.setMat4("projection", projection);

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

        skyFromSpace.use();
        skyFromSpace.setFloat("og_farPlaneDistance", MAXVIEWDISTANCE);
        skyFromSpace.setFloat("u_logarithmicDepthConstant", LOGDEPTHCONSTANT);
        skyFromSpace.setMat4("projection",projection);

        groundFromSpace.use();
        groundFromSpace.setFloat("og_farPlaneDistance", MAXVIEWDISTANCE);
        groundFromSpace.setFloat("u_logarithmicDepthConstant", LOGDEPTHCONSTANT);
        groundFromSpace.setMat4("projection",projection);

        planetAtmoShader.use();
        planetAtmoShader.setFloat("og_farPlaneDistance", MAXVIEWDISTANCE);
        planetAtmoShader.setFloat("u_logarithmicDepthConstant", LOGDEPTHCONSTANT);
        planetAtmoShader.setMat4("projection",projection);
        planetAtmoShader.setMat4("view",transformation.getViewMatrix(camera));

        for(SimulationObject object : toDraw){
            object.render(camera);
        }


        while(!lensGlows.isEmpty()){
            try {
                lensGlows.poll().render();
            } catch (Exception e) {
                e.printStackTrace();
            }
        }


        //simulationObject.render(camera);
        //simulationObject2.render(camera);
        Gdx.gl.glDisable(Gdx.gl.GL_CULL_FACE);
        Gdx.gl.glDisable(Gdx.gl.GL_DEPTH_TEST);
        Gdx.gl.glDisable(Gdx.gl.GL_BLEND);
    }

    public Vector2f projectPoint(Vector3f position){
        temp4f.set(worldSpaceToDeviceCoords(temp4f.set(position.x,position.y,position.z, 1)));
        temp4f.x*=screenWidth;
        temp4f.y*=screenHeight;

        //If the point is not visible, return null
        if (temp4f.w < 0){
            return null;
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

    public Shader getSkyFromSpaceShader() {
        return skyFromSpace;
    }

    public Shader getGroundFromSpaceShader() {
        return groundFromSpace;
    }

    public Shader getPlanetAtmoShader() {
        return planetAtmoShader;
    }

    @Override
    public void dispose() {

    }
}
